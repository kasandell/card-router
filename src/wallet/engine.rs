use uuid::Uuid;
use crate::adyen_service::checkout::service::{AdyenChargeServiceTrait, ChargeService};
use crate::api_error::ApiError;
use crate::credit_card_type::dao::{CreditCardDao, CreditCardDaoTrait};
use crate::service_error::ServiceError;
use crate::user::entity::User;
use crate::wallet::constant::WalletCardAttemptStatus;
use crate::wallet::dao::{WalletCardAttemptDao, WalletCardAttemtDaoTrait, WalletDao, WalletDaoTrait};
use crate::wallet::entity::{InsertableCardAttempt, NewCard, UpdateCardAttempt, Wallet, WalletCardAttempt};
use crate::wallet::request::{AddCardRequest, MatchAttemptRequest, RegisterAttemptRequest};
use adyen_checkout::models::PaymentRequestPaymentMethod as AdyenPaymentMethod;

// TODO: now that we make the api calls from the backend, we can consolidate the wallet card attempt creation
// and make the network call in one
pub struct Engine {
    pub credit_card_dao: Box<dyn CreditCardDaoTrait>,
    pub wallet_card_attempt_dao: Box<dyn WalletCardAttemtDaoTrait>,
    pub wallet_dao: Box<dyn WalletDaoTrait>,
    pub adyen_service: Box<dyn AdyenChargeServiceTrait>
}


impl Engine {
    pub fn new() -> Self {
        Self {
            credit_card_dao: Box::new(CreditCardDao::new()),
            wallet_card_attempt_dao: Box::new(WalletCardAttemptDao::new()),
            wallet_dao: Box::new(WalletDao::new()),
            adyen_service: Box::new(ChargeService::new())
        }
    }

    #[cfg(test)]
    pub fn new_with_services(
        credit_card_dao: Box<dyn CreditCardDaoTrait>,
        wallet_card_attempt_dao: Box<dyn WalletCardAttemtDaoTrait>,
        wallet_dao: Box<dyn WalletDaoTrait>,
        adyen_service: Box<dyn AdyenChargeServiceTrait>,
    ) -> Self {
        Self {
            credit_card_dao,
            wallet_card_attempt_dao,
            wallet_dao,
            adyen_service
        }
    }

    pub async fn register_attempt_and_send_card_to_adyen(
        &self,
        user: &User,
        request: &AddCardRequest
    ) -> Result<WalletCardAttempt, ServiceError> {

        println!("registering new attempt");
        let expected_reference_id = Uuid::new_v4();
        let credit_card = self.credit_card_dao.find_by_public_id(request.credit_card_type_public_id.clone())?;
        println!("found credit card id");
        let wca = self.wallet_card_attempt_dao.insert(
            InsertableCardAttempt {
                user_id: user.id,
                credit_card_id: credit_card.id,
                expected_reference_id: expected_reference_id.to_string()
            }
        )?;
        println!("CREATED WALLET CARED");
        let card_resp = self.adyen_service.add_card(
            &Uuid::new_v4().to_string(),
            &user,
            &expected_reference_id.to_string(),
            &AdyenPaymentMethod::from(request.payment_method.clone())
        ).await?;
        println!("GOT CARD RESPONSE");
        Ok(wca)
        // Err(ServiceError::new(500, "not implemented".to_string()))
    }

    pub async fn attempt_register_new_attempt(
        &self,
        user: &User,
        request: &RegisterAttemptRequest
    ) -> Result<WalletCardAttempt, ApiError> {
        let credit_card = self.credit_card_dao.find_by_public_id(request.credit_card_type_public_id.clone())?;
        let wca = self.wallet_card_attempt_dao.insert(
            InsertableCardAttempt {
                user_id: user.id,
                credit_card_id: credit_card.id,
                expected_reference_id: request.expected_reference_id.clone()
            }
        )?;
        Ok(wca)
    }

    pub async fn attempt_match(
        &self,
        request: &MatchAttemptRequest
    ) -> Result<Wallet, ServiceError> {
        let card_attempt = self.wallet_card_attempt_dao.find_by_reference_id(
            request.merchant_reference_id.clone()
        )?;
        info!("Found wallet card attempt id {}", card_attempt.id);

        if card_attempt.status.eq(&WalletCardAttemptStatus::MATCHED.as_str()) {
            return Err(ServiceError::new(409, "Card already matched".to_string()));
        }

        let update = self.wallet_card_attempt_dao.update_card(card_attempt.id, UpdateCardAttempt {
            recurring_detail_reference: request.psp_reference.clone(),
            psp_id: request.original_psp_reference.clone(),
            status: WalletCardAttemptStatus::MATCHED.as_str()
        })?;
        info!("Updated to matched: {}", &update.status);

        let created_card = self.wallet_dao.insert_card(
            NewCard {
                user_id: update.user_id,
                payment_method_id: request.psp_reference.clone(),
                credit_card_id: update.credit_card_id,
                wallet_card_attempt_id: update.id,
            }
        )?;
        info!("Created card: {}", &created_card.public_id);

        Ok(created_card)
    }
}