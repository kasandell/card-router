use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;
use adyen_checkout::models::payment_response::ResultCode;
use chrono::Utc;
use lazy_static::lazy_static;
use uuid::Uuid;
use crate::adyen::checkout::request::ChargeCardRequest;
use crate::adyen::checkout::service::{
    AdyenCheckoutService,
    AdyenChargeServiceTrait
};
use crate::asa::request::AsaRequest;
use crate::charge::entity::{
    ChargeEngineResult,
    ChargeCardAttemptResult
};
use crate::error::error_type::ErrorType;
use crate::footprint::request::ChargeThroughProxyRequest;
use crate::footprint::service::{FootprintService, FootprintServiceTrait};
use crate::passthrough_card::dao::{PassthroughCardDao, PassthroughCardDaoTrait};
use crate::passthrough_card::entity::PassthroughCard;
use crate::schema::registered_transactions::transaction_id;
use crate::error::service_error::ServiceError;
use crate::ledger::entity::{InnerChargeLedger, RegisteredTransaction, TransactionLedger, TransactionMetadata};
use crate::user::entity::User;
use crate::wallet::entity::Wallet;
use crate::ledger::service::{LedgerService as Ledger, LedgerServiceTrait};
use crate::user::dao::{UserDao, UserDaoTrait};

pub struct ChargeService {
    charge_service: Arc<dyn AdyenChargeServiceTrait>,
    passthrough_card_dao: Arc<dyn PassthroughCardDaoTrait>,
    user_dao: Arc<dyn UserDaoTrait>,
    ledger: Arc<dyn LedgerServiceTrait>,
    footprint_service: Arc<dyn FootprintServiceTrait>
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
impl ChargeService {

    #[tracing::instrument]
    pub fn new() -> Self {
       Self {
           charge_service: Arc::new(AdyenCheckoutService::new()),
           passthrough_card_dao: Arc::new(PassthroughCardDao{}),
           user_dao: Arc::new(UserDao{}),
           ledger: Arc::new(Ledger::new()),
           footprint_service: Arc::new(FootprintService::new())
       }
    }

    #[tracing::instrument(skip_all)]
    pub fn new_with_service(
        charge_service: Arc<dyn AdyenChargeServiceTrait>,
        passthrough_card_dao: Arc<dyn PassthroughCardDaoTrait>,
        user_dao: Arc<dyn UserDaoTrait>,
        ledger: Arc<dyn LedgerServiceTrait>,
        footprint_service: Arc<dyn FootprintServiceTrait>
    ) -> Self {
        Self {
            charge_service,
            passthrough_card_dao,
            user_dao,
            ledger,
            footprint_service
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn charge_from_asa_request(
        self: Arc<Self>,
        request: &AsaRequest,
        wallet: &Vec<Wallet>,
        passthrough_card: &PassthroughCard,
        user: &User,
    ) -> Result<(ChargeEngineResult, Option<TransactionLedger>), ServiceError> {
        tracing::info!("Starting charge");
        let metadata = TransactionMetadata::convert(&request)?;
        let card = request.card.clone().ok_or(ServiceError::new(ErrorType::BadRequest, "expect card"))?;
        let token = card.token.clone().ok_or(ServiceError::new(ErrorType::BadRequest, "expect token"))?;
        //let passthrough_card = self.passthrough_card_dao.clone().get_by_token(token).await?;
        //let user = self.user_dao.clone().find_by_internal_id(passthrough_card.user_id).await?;
        tracing::info!("Registering txn");
        let rtx = self.ledger.clone().register_transaction_for_user(
            &user,
            &metadata
        ).await?;

        tracing::info!("Charging wallet");
        let (charge_result, ledger) = self.clone().charge_wallet(
            &user,
            wallet,
            &metadata,
            &rtx
        ).await?;
        return match charge_result {
            ChargeEngineResult::Approved => {
                if let Some(ledger) = ledger {
                    // TODO: should verify that this is success
                    let outer_successs = self.ledger.clone().register_successful_outer_charge(
                        &rtx,
                        &metadata,
                        &passthrough_card
                    ).await?;

                    let full_txn = self.ledger.clone().register_full_transaction(
                        &rtx,
                        &ledger,
                        &outer_successs
                    ).await?;
                    Ok((charge_result, Some(full_txn)))

                } else {
                    self.ledger.clone().register_failed_outer_charge(
                        &rtx,
                        &metadata,
                        &passthrough_card
                    ).await?;
                    Err(ServiceError::new(ErrorType::InternalServerError, "Approved inner charge with no ledger entry, should not be possible"))
                }
            },
            _ => {
                self.ledger.clone().register_failed_outer_charge(
                    &rtx,
                    &metadata,
                    &passthrough_card
                ).await?;
                Ok((charge_result, None))
            }
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn charge_wallet(
        self: Arc<Self>,
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
        tracing::info!("Charging {} cards for user={}", wallet.len(), user.id);
        for card in wallet {
            if success_charge { break; }
            if let Ok((charge_attempt, ledger)) = self.clone().charge_card_with_cleanup(
                idempotency_key,
                card,
                user,
                transaction_metadata,
                registered_transaction
            ).await {
                tracing::info!("Successfully charged card={} for user={}", card.id, user.id);
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

    #[tracing::instrument(skip(self))]
    pub async fn charge_card_with_cleanup(
        self: Arc<Self>,
        idempotency_key: Uuid,
        card: &Wallet,
        user: &User,
        transaction_metadata: &TransactionMetadata,
        registered_transaction: &RegisteredTransaction
    ) -> Result<(ChargeCardAttemptResult, Option<InnerChargeLedger>), ServiceError> {
        let resp = self.charge_service.clone().charge_card_on_file(
            &ChargeCardRequest {
                amount_cents: transaction_metadata.amount_cents,
                mcc: &transaction_metadata.mcc,
                payment_method_id: &card.payment_method_id,
                customer_public_id: &user.public_id,
                idempotency_key: &idempotency_key,
                reference: &Uuid::new_v4().to_string(),
                statement: &transaction_metadata.memo,
            }
        ).await;
        /*
        let resp = self.footprint_service.clone().proxy_adyen_payment_request(
            &ChargeThroughProxyRequest {
                amount_cents: transaction_metadata.amount_cents as i32, // TODO: edit model to be i32
                mcc: &transaction_metadata.mcc,
                payment_method_id: &card.payment_method_id,
                customer_public_id: &user.public_id.to_string(),
                idempotency_key: &idempotency_key,
                reference: &Uuid::new_v4().to_string(),
                statement: &transaction_metadata.memo
            }
        ).await;
         */

        if let Ok(response) = resp {
            if let Some(code) = response.result_code {
                tracing::info!("Checkout returned code={:?} for card={} user={}", code, card.id, user.id);
                if ACCEPTABLE_STATUSES.contains(&code) {
                    tracing::info!("Charged card={} for user={}", card.id, user.id);
                    let ledger_entry = self.ledger.clone().register_successful_inner_charge(
                        registered_transaction,
                        transaction_metadata,
                        card
                    ).await?;
                    return Ok((ChargeCardAttemptResult::from(code), Some(ledger_entry)));


                    //add to ledger
                } else if FINAL_STATE_ERROR_CODES.contains(&code) {
                    tracing::warn!("Error charging card={} for user={}", card.id, user.id);
                    let ledger_entry = self.ledger.clone().register_failed_inner_charge(
                        registered_transaction,
                        transaction_metadata,
                        card
                    ).await?;
                    return Ok((ChargeCardAttemptResult::Denied, Some(ledger_entry)));
                    //can safely bypass this branch
                } else {
                    tracing::warn!("Intermediate state needs cleanup for card={} for user={}", card.id, user.id);
                    if let Some(psp) = response.psp_reference {
                        // TODO: move this call to proxy
                        let cancel = self.charge_service.clone().cancel_transaction(
                            &psp
                        ).await;
                        if let Ok(cancel) = cancel {
                            tracing::info!("Cancelled with status: {:?}", cancel.status);
                            let ledger_entry = self.ledger.clone().register_failed_inner_charge(
                                registered_transaction,
                                transaction_metadata,
                                card
                            ).await?;
                            return Ok((ChargeCardAttemptResult::PartialCancelSucceeded, Some(ledger_entry)));
                            //cancel received. block on webhook response?
                        } else {
                            tracing::error!("Error cancelling unsuccessful payment with psp={}", psp);
                            let ledger_entry = self.ledger.clone().register_failed_inner_charge(
                                registered_transaction,
                                transaction_metadata,
                                card
                            ).await?;
                            return Ok((ChargeCardAttemptResult::PartialCancelFailed, Some(ledger_entry)));
                            // error cancelling. figure out what to do
                        }
                    }
                }
            }
        }
        let ledger_entry = self.ledger.clone().register_failed_inner_charge(
            registered_transaction,
            transaction_metadata,
            card
        ).await?;
        Ok((ChargeCardAttemptResult::Denied, Some(ledger_entry)))
    }
}