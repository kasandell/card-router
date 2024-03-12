use std::sync::Arc;
use uuid::Uuid;
use crate::adyen::checkout::service::{AdyenChargeServiceTrait, ChargeService};
use crate::api_error::ApiError;
use crate::credit_card_type::dao::{CreditCardDao, CreditCardDaoTrait};
use crate::service_error::ServiceError;
use crate::user::entity::User;
use crate::wallet::constant::WalletCardAttemptStatus;
use crate::wallet::dao::{WalletCardAttemptDao, WalletCardAttemtDaoTrait, WalletDao, WalletDaoTrait};
use crate::wallet::entity::{InsertableCardAttempt, NewCard, UpdateCardAttempt, Wallet, WalletCardAttempt};
use crate::wallet::request::{AddCardRequest, MatchAttemptRequest, RegisterAttemptRequest};
use adyen_checkout::models::{PaymentRequestPaymentMethod as AdyenPaymentMethod, PaymentResponse};
use crate::constant::constant;
use crate::error_type::ErrorType;

// TODO: now that we make the api calls from the backend, we can consolidate the wallet card attempt creation
// and make the network call in one
pub struct WalletService {
    pub credit_card_dao: Arc<dyn CreditCardDaoTrait>,
    pub wallet_card_attempt_dao: Arc<dyn WalletCardAttemtDaoTrait>,
    pub wallet_dao: Arc<dyn WalletDaoTrait>,
    pub adyen_service: Arc<dyn AdyenChargeServiceTrait>
}


impl WalletService {
    pub fn new() -> Self {
        Self {
            credit_card_dao: Arc::new(CreditCardDao::new()),
            wallet_card_attempt_dao: Arc::new(WalletCardAttemptDao::new()),
            wallet_dao: Arc::new(WalletDao::new()),
            adyen_service: Arc::new(ChargeService::new())
        }
    }

    pub fn new_with_services(
        credit_card_dao: Arc<dyn CreditCardDaoTrait>,
        wallet_card_attempt_dao: Arc<dyn WalletCardAttemtDaoTrait>,
        wallet_dao: Arc<dyn WalletDaoTrait>,
        adyen_service: Arc<dyn AdyenChargeServiceTrait>,
    ) -> Self {
        Self {
            credit_card_dao,
            wallet_card_attempt_dao,
            wallet_dao,
            adyen_service
        }
    }

    pub async fn register_attempt_and_send_card_to_adyen(
        self: Arc<Self>,
        user: &User,
        request: &AddCardRequest
    ) -> Result<(WalletCardAttempt, PaymentResponse), ServiceError> {

        println!("registering new attempt");
        let expected_reference_id = Uuid::new_v4();
        let credit_card = self.credit_card_dao.clone().find_by_public_id(&request.credit_card_type_public_id).await?;
        println!("found credit card id");
        let wca = self.wallet_card_attempt_dao.clone().insert(
            &InsertableCardAttempt {
                user_id: user.id,
                credit_card_id: credit_card.id,
                expected_reference_id: &expected_reference_id.to_string()
            }
        ).await?;
        println!("CREATED WALLET CARED");
        let card_resp = self.adyen_service.clone().add_card(
            &Uuid::new_v4().to_string(),
            &user,
            &expected_reference_id.to_string(),
            &AdyenPaymentMethod::from(request.payment_method.clone())
        ).await?;
        println!("GOT CARD RESPONSE");
        println!("{:?}", card_resp);
        Ok((wca, card_resp))
    }

    pub async fn attempt_match_from_response(
        self: Arc<Self>,
        payment_response: &PaymentResponse
    ) -> Result<Option<Wallet>, ServiceError> {
        let Some(Some(additional_data)) = payment_response.additional_data.clone() else { return Ok(None); };
        let Some(recurring_detail) = additional_data.get(constant::RECURRING_DETAIL_KEY).cloned() else { return Ok(None); };
        let Some(merchant_reference) = payment_response.merchant_reference.clone() else { return Ok(None); };
        let Some(original_reference) = payment_response.psp_reference.clone() else { return Ok(None); };

        let match_attempt = self.clone().attempt_match(
            &MatchAttemptRequest {
                merchant_reference_id: merchant_reference,
                original_psp_reference: original_reference,
                psp_reference: recurring_detail
            }
        ).await;

        // we want to swallow errors. this is optional
        match match_attempt {
            Ok(wallet) => Ok(Some(wallet)),
            Err(err) => Ok(None)
        }
    }

    pub async fn attempt_register_new_attempt(
        self: Arc<Self>,
        user: &User,
        request: &RegisterAttemptRequest
    ) -> Result<WalletCardAttempt, ApiError> {
        let credit_card = self.credit_card_dao.clone().find_by_public_id(&request.credit_card_type_public_id).await?;
        let wca = self.wallet_card_attempt_dao.clone().insert(
            &InsertableCardAttempt {
                user_id: user.id,
                credit_card_id: credit_card.id,
                expected_reference_id: &request.expected_reference_id
            }
        ).await?;
        Ok(wca)
    }

    pub async fn attempt_match(
        self: Arc<Self>,
        request: &MatchAttemptRequest
    ) -> Result<Wallet, ServiceError> {
        let card_attempt = self.wallet_card_attempt_dao.clone().find_by_reference_id(
            &request.merchant_reference_id
        ).await?;
        info!("Found wallet card attempt id {}", card_attempt.id);

        if card_attempt.status.eq(&WalletCardAttemptStatus::Matched) {
            return Err(ServiceError::new(ErrorType::Conflict, "Card already matched"));
        }

        let update = self.wallet_card_attempt_dao.clone().update_card(card_attempt.id, &UpdateCardAttempt {
            recurring_detail_reference: &request.psp_reference,
            psp_id: &request.original_psp_reference,
            status: WalletCardAttemptStatus::Matched
        }).await?;
        info!("Updated to matched: {}", &update.status);

        let created_card = self.wallet_dao.clone().insert_card(
            &NewCard {
                user_id: update.user_id,
                payment_method_id: &request.psp_reference,
                credit_card_id: update.credit_card_id,
                wallet_card_attempt_id: update.id,
            }
        ).await?;
        info!("Created card: {}", &created_card.public_id);

        Ok(created_card)
    }
}