use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use uuid::Uuid;
use crate::adyen::checkout::service::{AdyenChargeServiceTrait, AdyenCheckoutService};
use crate::credit_card_type::dao::{CreditCardDao, CreditCardDaoTrait};
use crate::error::error::ServiceError;
use crate::user::entity::User;
use crate::wallet::constant::WalletCardAttemptStatus;
use crate::wallet::dao::{WalletCardAttemptDao, WalletCardAttemtDaoTrait, WalletDao, WalletDaoTrait};
use crate::wallet::entity::{InsertableCardAttempt, NewCard, UpdateCardAttempt, Wallet, WalletCardAttempt};
use crate::wallet::request::{MatchRequest, RegisterAttemptRequest};

use crate::footprint::service::{FootprintService, FootprintServiceTrait};
use crate::wallet::response::WalletCardAttemptResponse;

// TODO: now that we make the api calls from the backend, we can consolidate the wallet card attempt creation
// and make the network call in one
pub struct WalletService {
    pub credit_card_dao: Arc<dyn CreditCardDaoTrait>,
    pub wallet_card_attempt_dao: Arc<dyn WalletCardAttemtDaoTrait>,
    pub wallet_dao: Arc<dyn WalletDaoTrait>,
    pub adyen_service: Arc<dyn AdyenChargeServiceTrait>,
    pub footprint_service: Arc<dyn FootprintServiceTrait>
}

impl WalletService {
    #[tracing::instrument(skip_all)]
    pub fn new() -> Self {
        Self {
            credit_card_dao: Arc::new(CreditCardDao::new()),
            wallet_card_attempt_dao: Arc::new(WalletCardAttemptDao::new()),
            wallet_dao: Arc::new(WalletDao::new()),
            adyen_service: Arc::new(AdyenCheckoutService::new()),
            footprint_service: Arc::new(FootprintService::new())
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn new_with_services(
        credit_card_dao: Arc<dyn CreditCardDaoTrait>,
        wallet_card_attempt_dao: Arc<dyn WalletCardAttemtDaoTrait>,
        wallet_dao: Arc<dyn WalletDaoTrait>,
        adyen_service: Arc<dyn AdyenChargeServiceTrait>,
        footprint_service: Arc<dyn FootprintServiceTrait>
    ) -> Self {
        Self {
            credit_card_dao,
            wallet_card_attempt_dao,
            wallet_dao,
            adyen_service,
            footprint_service
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn register_new_attempt(
        self: Arc<Self>,
        user: &User,
        request: &RegisterAttemptRequest
    ) -> Result<WalletCardAttemptResponse, ServiceError> {
        let credit_card = self.credit_card_dao.clone().find_by_public_id(&request.credit_card_type_public_id).await?;
        let wca = self.wallet_card_attempt_dao.clone().insert(
            &InsertableCardAttempt {
                user_id: user.id,
                credit_card_id: credit_card.id,
                expected_reference_id: &Uuid::new_v4().to_string()
            }
        ).await?;

        let token = self.footprint_service.clone().create_client_token(
            &user,
            &wca.expected_reference_id
        ).await?;

        Ok(WalletCardAttemptResponse {
            reference_id: wca.expected_reference_id,
            token: token.token,
            expires_at: token.expires_at,
        })
    }

    #[tracing::instrument(skip(self))]
    pub async fn match_card(
        self: Arc<Self>,
        user: &User,
        request: &MatchRequest
    ) -> Result<Wallet, ServiceError> {
        let card_attempt = self.wallet_card_attempt_dao.clone().find_by_reference_id(
            &request.reference_id
        ).await?;
        tracing::info!("Found wallet card attempt id {}", card_attempt.id);

        if card_attempt.status.eq(&WalletCardAttemptStatus::Matched) {
            return Err(ServiceError::Conflict(Box::new("Card already matched")));
        }
        if user.id != card_attempt.user_id {
            return Err(ServiceError::Unauthorized(Box::new("User not owner of attempt")));
        }

        let update = self.wallet_card_attempt_dao.clone().update_card(card_attempt.id, &UpdateCardAttempt {
            status: WalletCardAttemptStatus::Matched
        }).await?;
        tracing::info!("Updated to matched: {}", &update.status);

        let created_card = self.wallet_dao.clone().insert_card(
            &NewCard {
                user_id: update.user_id,
                payment_method_id: &request.reference_id,
                credit_card_id: update.credit_card_id,
                wallet_card_attempt_id: update.id,
            }
        ).await?;
        tracing::info!("Created card: {}", &created_card.public_id);
        Ok(created_card)
    }

}