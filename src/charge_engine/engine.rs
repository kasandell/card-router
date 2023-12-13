use std::collections::HashSet;
use adyen_checkout::models::payment_response::ResultCode;
use lazy_static::lazy_static;
use uuid::Uuid;
use crate::adyen_service::checkout::request::ChargeCardRequest;
use crate::adyen_service::checkout::service::{
    ChargeService,
    AdyenChargeServiceTrait
};
use crate::charge_engine::error::Error;
use crate::user::entity::User;
use crate::wallet::entity::Wallet;

pub struct Engine {
    charge_service: Box<dyn AdyenChargeServiceTrait>,
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
           charge_service: Box::new(ChargeService{})
       }
    }

    #[cfg(test)]
    pub fn new_with_service(charge_service: Box<dyn AdyenChargeServiceTrait>) -> Self {
        Self {
            charge_service
        }
    }

    pub async fn charge_wallet(
        &self,
        user: &User,
        wallet: &Vec<Wallet>,
        amount_cents: i32,
        mcc: &str,
        statement: &str,
    ) -> Result<bool, Error> {
        // iterate through the users wallet, charging one and ONLY ONE card
        let idempotency_key = Uuid::new_v4();
        let mut success_charge = false;
        info!("Charging {} cards for user={}", wallet.len(), user.id);
        for card in wallet {
            if success_charge { break; }
            if let Ok(charge_attempt) = self.charge_card_with_cleanup(
                idempotency_key,
                card,
                user,
                amount_cents,
                mcc,
                statement,
            ).await {
                info!("Successfully charged card={} for user={}", card.id, user.id);
                success_charge = charge_attempt;
            }
        }
        Ok(success_charge)
    }

    pub async fn charge_card_with_cleanup(
        &self,
        idempotency_key: Uuid,
        card: &Wallet,
        user: &User,
        amount_cents: i32,
        mcc: &str,
        statement: &str,
    ) -> Result<bool, Error> {
        let resp = self.charge_service.charge_card_on_file(
            ChargeCardRequest {
                amount_cents: amount_cents,
                mcc: mcc.to_string(),
                payment_method_id: card.payment_method_id.clone(),
                customer_public_id: user.public_id.clone(),
                idempotency_key: idempotency_key.to_string(),
                reference: Uuid::new_v4().to_string(), // TODO: this will later be done with what we put in ledger for attempts
                statement: statement.to_string(),
            }
        );

        if let Ok(response) = resp {
            if let Some(code) = response.result_code {
                info!("Checkout returned code={:?} for card={} user={}", code, card.id, user.id);
                if ACCEPTABLE_STATUSES.contains(&code) {
                    info!("Charged card={} for user={}", card.id, user.id);
                    return Ok(true);
                    //add to ledger
                } else if FINAL_STATE_ERROR_CODES.contains(&code) {
                    warn!("Error charging card={} for user={}", card.id, user.id);
                    //can safely bypass this branch
                } else {
                    warn!("Intermediate state needs cleanup for card={} for user={}", card.id, user.id);
                    if let Some(psp) = response.psp_reference {
                        let cancel = self.charge_service.cancel_transaction(
                            &psp
                        );
                        if let Ok(cancel) = cancel {
                            info!("Cancelled with status: {:?}", cancel.status);
                            //cancel received. block on webhook response?
                        } else {
                            error!("Error cancelling unsuccessful payment with psp={}", psp)
                            // error cancelling. figure out what to do
                        }
                    }
                }
            }
        }
        Ok(false)
    }
}