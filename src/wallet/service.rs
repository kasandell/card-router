use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;
use crate::adyen::checkout::service::{AdyenChargeServiceTrait, AdyenCheckoutService};
use crate::credit_card_type::service::CreditCardServiceTrait;
use crate::error::data_error::DataError;
use crate::user::model::UserModel as User;
use crate::wallet::constant::WalletCardAttemptStatus;
use crate::wallet::dao::{WalletCardAttemptDao, WalletCardAttemtDaoTrait, WalletDao, WalletDaoTrait};
use crate::wallet::entity::{InsertableCardAttempt, InsertableCard, UpdateCardAttempt, Wallet, WalletCardAttempt, WalletDetail};
use crate::wallet::request::{MatchRequest, RegisterAttemptRequest};

use crate::footprint::service::{FootprintService, FootprintServiceTrait};
use crate::wallet::error::WalletError;
use crate::wallet::model::{WalletModel, WalletWithExtraInfoModel};
use crate::wallet::response::WalletCardAttemptResponse;


#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait WalletServiceTrait {
    async fn match_card(
        self: Arc<Self>,
        user: &User,
        request: &MatchRequest
    ) -> Result<WalletModel, WalletError>;

    async fn register_new_attempt(
        self: Arc<Self>,
        user: &User,
        request: &RegisterAttemptRequest
    ) -> Result<WalletCardAttemptResponse, WalletError>;

    async fn find_all_for_user(self: Arc<Self>, user: &User) -> Result<Vec<WalletModel>, WalletError>;
    async fn find_all_for_user_with_card_info(self: Arc<Self>, user: &User) -> Result<Vec<WalletWithExtraInfoModel>, WalletError>;
}

// TODO: now that we make the api calls from the backend, we can consolidate the wallet card attempt creation
// and make the network call in one
pub struct WalletService {
    credit_card_service: Arc<dyn CreditCardServiceTrait>,
    wallet_card_attempt_dao: Arc<dyn WalletCardAttemtDaoTrait>,
    wallet_dao: Arc<dyn WalletDaoTrait>,
    footprint_service: Arc<dyn FootprintServiceTrait>
}

impl WalletService {
    #[cfg_attr(feature="trace-detail", tracing::instrument(skip_all))]
    pub fn new_with_services(
        credit_card_service: Arc<dyn CreditCardServiceTrait>,
        footprint_service: Arc<dyn FootprintServiceTrait>
    ) -> Self {
        Self {
            credit_card_service,
            wallet_card_attempt_dao: Arc::new(WalletCardAttemptDao::new()),
            wallet_dao: Arc::new(WalletDao::new()),
            footprint_service
        }
    }
}

#[async_trait(?Send)]
impl WalletServiceTrait for WalletService {

    #[tracing::instrument(skip(self))]
    async fn register_new_attempt(
        self: Arc<Self>,
        user: &User,
        request: &RegisterAttemptRequest
    ) -> Result<WalletCardAttemptResponse, WalletError> {
        tracing::info!("Registering new card attempt for user_id={}", &user.id);
        let credit_card = self.credit_card_service.clone().find_by_public_id(&request.credit_card_type_public_id)
            .await.map_err(|e| {
            tracing::error!("Unexpected error finding credit card by public_id={} error={:?}", &request.credit_card_type_public_id, &e);
            WalletError::Unexpected(e.into())
        })?;
        tracing::info!("inserting card attempt");
        let wca = self.wallet_card_attempt_dao.clone().insert(
            &InsertableCardAttempt {
                user_id: user.id,
                credit_card_id: credit_card.id,
                expected_reference_id: &Uuid::new_v4().to_string()
            }
        ).await?;
        tracing::info!("Inserted attempt with id={}", wca.id);
        tracing::info!("Creating footprint token for attempt");
        let token = self.footprint_service.clone().create_client_token(
            &user,
            &wca.expected_reference_id
        ).await.map_err(|e| {
            tracing::error!("Error creating footprint token={:?}", &e);
            WalletError::Unexpected(e.into())
        })?;

        Ok(WalletCardAttemptResponse {
            reference_id: wca.expected_reference_id,
            token: token.token,
            expires_at: token.expires_at,
        })
    }

    #[tracing::instrument(skip(self))]
    async fn match_card(
        self: Arc<Self>,
        user: &User,
        request: &MatchRequest
    ) -> Result<WalletModel, WalletError> {
        tracing::info!("Matching card for user_id={} reference_id={}", &user.id, &request.reference_id);
        let card_attempt = self.wallet_card_attempt_dao.clone().find_by_reference_id(
            &request.reference_id
        ).await?;
        tracing::info!("Found wallet card attempt id {}", card_attempt.id);

        if card_attempt.status.eq(&WalletCardAttemptStatus::Matched) {
            tracing::error!("Card already matched");
            return Err(WalletError::Conflict("Card already matched".into()))
        }
        if user.id != card_attempt.user_id {
            tracing::error!("User is not owner of attempt");
            return Err(WalletError::Unauthorized("User is not owner of attempt".into()))
        }

        tracing::info!("Creating card");
        let created_card = self.wallet_dao.clone().insert_card(
            &InsertableCard {
                user_id: card_attempt.user_id,
                payment_method_id: &request.reference_id,
                credit_card_id: card_attempt.credit_card_id,
                wallet_card_attempt_id: card_attempt.id,
            }
        ).await?;
        tracing::info!("Created card: {}", &created_card.public_id);
        let update = self.wallet_card_attempt_dao.clone().update_card(card_attempt.id, &UpdateCardAttempt {
            status: WalletCardAttemptStatus::Matched
        }).await?;
        tracing::info!("Updated to matched: {}", &update.status);
        Ok(created_card.into())
    }

    #[tracing::instrument(skip(self))]
    async fn find_all_for_user(self: Arc<Self>, user: &User) -> Result<Vec<WalletModel>, WalletError> {
        tracing::info!("Finding all cards for user_id={}", &user.id);
        Ok(
            self.wallet_dao.clone().find_all_for_user(user).await?
                .into_iter()
                .map(|e| e.into())
                .collect()
        )
    }

    #[tracing::instrument(skip(self))]
    async fn find_all_for_user_with_card_info(self: Arc<Self>, user: &User) -> Result<Vec<WalletWithExtraInfoModel>, WalletError> {
        tracing::info!("Finding all cards with extra info for user_id={}", &user.id);
        Ok(
            self.wallet_dao.clone().find_all_for_user_with_card_info(user).await?
                .into_iter()
                .map(|e| e.into())
                .collect()
        )
    }

}