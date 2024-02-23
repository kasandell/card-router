use std::collections::HashSet;
use adyen_checkout::models::payment_response::ResultCode;
use lazy_static::lazy_static;
use uuid::Uuid;
use crate::adyen_service::checkout::request::ChargeCardRequest;
use crate::adyen_service::checkout::service::{
    ChargeService,
    AdyenChargeServiceTrait
};
use crate::asa::request::AsaRequest;
use crate::charge_engine::entity::{
    ChargeEngineResult,
    ChargeCardAttemptResult
};
use crate::passthrough_card::dao::{PassthroughCardDao, PassthroughCardDaoTrait};
use crate::service_error::ServiceError;
use crate::transaction::entity::{InnerChargeLedger, RegisteredTransaction, TransactionLedger, TransactionMetadata};
use crate::user::entity::User;
use crate::wallet::entity::Wallet;
use crate::transaction::engine::{Engine as Ledger, TransactionEngineTrait};
use crate::user::dao::{UserDao, UserDaoTrait};

pub struct Engine {
    charge_service: Box<dyn AdyenChargeServiceTrait>,
    passthrough_card_dao: Box<dyn PassthroughCardDaoTrait>,
    user_dao: Box<dyn UserDaoTrait>,
    ledger: Box<dyn TransactionEngineTrait>
}

lazy_static! {
    static ref ACCEPTABLE_STATUSES: HashSet<ResultCode> = HashSet::from(
        [
            ResultCode::Authorised
        ]
    );
    static ref FINAL_STATE_ERROR_CODES: HashSet<ResultCode> = HashSet::from(
        [
            ResultCode::Cancelled,
            ResultCode::Error,
            ResultCode::Refused
        ]
    );
}

// TODO: probably need this to be a threadsafe singleton to avoid reinit everywhere
impl Engine {

    pub fn new() -> Self {
       Self {
           charge_service: Box::new(ChargeService{}),
           passthrough_card_dao: Box::new(PassthroughCardDao{}),
           user_dao: Box::new(UserDao{}),
           ledger: Box::new(Ledger::new())
       }
    }

    #[cfg(test)]
    pub fn new_with_service(
        charge_service: Box<dyn AdyenChargeServiceTrait>,
        passthrough_card_dao: Box<dyn PassthroughCardDaoTrait>,
        user_dao: Box<dyn UserDaoTrait>,
        ledger: Box<dyn TransactionEngineTrait>,
    ) -> Self {
        Self {
            charge_service,
            passthrough_card_dao,
            user_dao,
            ledger
        }
    }

    pub async fn charge_from_asa_request(
        &self,
        request: &AsaRequest,
        wallet: &Vec<Wallet>
    ) -> Result<(ChargeEngineResult, Option<TransactionLedger>), ServiceError> {
        let metadata = TransactionMetadata::convert(&request)?;
        let token = request.token.clone().ok_or(ServiceError::new(400, "expect token".to_string()))?;
        let passthrough_card = self.passthrough_card_dao.get_by_token(token)?;
        let user = self.user_dao.find_by_internal_id(passthrough_card.user_id)?;
        let rtx = self.ledger.register_transaction_for_user(
            &user,
            &metadata
        )?;

        let (charge_result, ledger) = self.charge_wallet(
            &user,
            wallet,
            &metadata,
            &rtx
        ).await?;
        return match charge_result {
            ChargeEngineResult::Approved => {
                if let Some(ledger) = ledger {
                    // TODO: should verify that this is success
                    let outer_successs = self.ledger.register_successful_outer_charge(
                        &rtx,
                        &metadata,
                        &passthrough_card
                    )?;

                    let full_txn = self.ledger.register_full_transaction(
                        &rtx,
                        &ledger,
                        &outer_successs
                    )?;
                    Ok((charge_result, Some(full_txn)))

                } else {
                    self.ledger.register_failed_outer_charge(
                        &rtx,
                        &metadata,
                        &passthrough_card
                    )?;
                    Err(ServiceError::new(500, "Approved inner charge with no ledger entry, should not be possible".to_string()))
                }
            },
            _ => {
                self.ledger.register_failed_outer_charge(
                    &rtx,
                    &metadata,
                    &passthrough_card
                )?;
                Ok((charge_result, None))

            }
        }
    }

    pub async fn charge_wallet(
        &self,
        user: &User,
        wallet: &Vec<Wallet>,
        transaction_metadata: &TransactionMetadata,
        registered_transaction: &RegisteredTransaction
    ) -> Result<(ChargeEngineResult, Option<InnerChargeLedger>), ServiceError> {
        // iterate through the users wallet, charging one and ONLY ONE card
        let idempotency_key = Uuid::new_v4();
        let mut success_charge = false;
        let mut codes : Vec<ChargeCardAttemptResult> = vec![];
        let mut ledger_res: Option<InnerChargeLedger> = None;
        info!("Charging {} cards for user={}", wallet.len(), user.id);
        for card in wallet {
            if success_charge { break; }
            if let Ok((charge_attempt, ledger)) = self.charge_card_with_cleanup(
                idempotency_key,
                card,
                user,
                transaction_metadata,
                registered_transaction
            ).await {
                info!("Successfully charged card={} for user={}", card.id, user.id);
                success_charge = bool::from(&charge_attempt);
                ledger_res = ledger;
                codes.push(charge_attempt)

            }
        }
        if success_charge {
            Ok((ChargeEngineResult::Approved, ledger_res))

        } else {
            Ok((ChargeEngineResult::Denied, ledger_res))
        }
    }

    pub async fn charge_card_with_cleanup(
        &self,
        idempotency_key: Uuid,
        card: &Wallet,
        user: &User,
        transaction_metadata: &TransactionMetadata,
        registered_transaction: &RegisteredTransaction
    ) -> Result<(ChargeCardAttemptResult, Option<InnerChargeLedger>), ServiceError> {
        let resp = self.charge_service.charge_card_on_file(
            ChargeCardRequest {
                amount_cents: transaction_metadata.amount_cents as i32, // TODO: edit model to be i32
                mcc: transaction_metadata.mcc.to_string(),
                payment_method_id: card.payment_method_id.clone(),
                customer_public_id: user.public_id.clone(),
                idempotency_key: idempotency_key.to_string(),
                reference: Uuid::new_v4().to_string(), // TODO: this will later be done with what we put in ledger for attempts
                statement: transaction_metadata.memo.clone(),
            }
        );

        if let Ok(response) = resp {
            if let Some(code) = response.result_code {
                info!("Checkout returned code={:?} for card={} user={}", code, card.id, user.id);
                if ACCEPTABLE_STATUSES.contains(&code) {
                    info!("Charged card={} for user={}", card.id, user.id);
                    let ledger_entry = self.ledger.register_successful_inner_charge(
                        registered_transaction,
                        transaction_metadata,
                        card
                    )?;
                    return Ok((ChargeCardAttemptResult::from(code), Some(ledger_entry)));


                    //add to ledger
                } else if FINAL_STATE_ERROR_CODES.contains(&code) {
                    warn!("Error charging card={} for user={}", card.id, user.id);
                    let ledger_entry = self.ledger.register_failed_inner_charge(
                        registered_transaction,
                        transaction_metadata,
                        card
                    )?;
                    return Ok((ChargeCardAttemptResult::Denied, Some(ledger_entry)));
                    //can safely bypass this branch
                } else {
                    warn!("Intermediate state needs cleanup for card={} for user={}", card.id, user.id);
                    if let Some(psp) = response.psp_reference {
                        let cancel = self.charge_service.cancel_transaction(
                            &psp
                        );
                        if let Ok(cancel) = cancel {
                            info!("Cancelled with status: {:?}", cancel.status);
                            let ledger_entry = self.ledger.register_failed_inner_charge(
                                registered_transaction,
                                transaction_metadata,
                                card
                            )?;
                            return Ok((ChargeCardAttemptResult::PartialCancelSucceeded, Some(ledger_entry)));
                            //cancel received. block on webhook response?
                        } else {
                            error!("Error cancelling unsuccessful payment with psp={}", psp);
                            let ledger_entry = self.ledger.register_failed_inner_charge(
                                registered_transaction,
                                transaction_metadata,
                                card
                            )?;
                            return Ok((ChargeCardAttemptResult::PartialCancelFailed, Some(ledger_entry)));
                            // error cancelling. figure out what to do
                        }
                    }
                }
            }
        }
        let ledger_entry = self.ledger.register_failed_inner_charge(
            registered_transaction,
            transaction_metadata,
            card
        )?;
        Ok((ChargeCardAttemptResult::Denied, Some(ledger_entry)))
    }
}