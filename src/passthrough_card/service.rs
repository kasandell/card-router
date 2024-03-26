use std::sync::Arc;
use uuid::Uuid;
use crate::user::model::UserModel as User;
use crate::passthrough_card::constant::{PassthroughCardStatus, PassthroughCardType};
use crate::passthrough_card::entity::{InsertablePassthroughCard, PassthroughCard};
use lithic_client::models::card::Card;
#[cfg(test)]
use mockall::automock;

use async_trait::async_trait;
use super::error::PassthroughCardError;

use crate::lithic::{
    service::LithicService,
    service::LithicServiceTrait,
};
use crate::passthrough_card::crypto::encrypt_pin;
use crate::passthrough_card::dao::{PassthroughCardDao, PassthroughCardDaoTrait};
use crate::passthrough_card::model::PassthroughCardModel;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait PassthroughCardServiceTrait {
    async fn issue_card_to_user(
        self: Arc<Self>,
        user: &User,
        pin: &str
    ) -> Result<PassthroughCardModel, PassthroughCardError>;


    async fn update_card_status(
        self: Arc<Self>,
        user: &User,
        status: PassthroughCardStatus
    ) -> Result<(), PassthroughCardError>;

    async fn get_by_token(self: Arc<Self>, token: &str) -> Result<PassthroughCardModel, PassthroughCardError>;


}

pub struct PassthroughCardService {
    lithic_service: Arc<dyn LithicServiceTrait>,
    passthrough_card_dao: Arc<dyn PassthroughCardDaoTrait>
}

#[async_trait(?Send)]
impl PassthroughCardServiceTrait for PassthroughCardService {
    #[tracing::instrument(skip(self))]
    async fn issue_card_to_user(
        self: Arc<Self>,
        user: &User,
        pin: &str
    ) -> Result<PassthroughCardModel, PassthroughCardError> {
        let has_active = self.clone().user_has_active_card(&user).await?;
        if has_active {
            return Err(PassthroughCardError::ActiveCardExists("User has active card already".into()))
        }
        let idempotency_key = Uuid::new_v4();
        let pin_encoded = encrypt_pin(pin);
        let lithic_card = self.lithic_service.clone().create_card(
            &pin_encoded,
            &idempotency_key
        ).await.map_err(|e| PassthroughCardError::IssueCard(e.into()))?;
        let token = lithic_card.token.to_string();
        let card = InsertablePassthroughCard::try_from((lithic_card, user))?;
        let inserted_card = self.passthrough_card_dao.clone().create(
            card
        ).await;
        return match inserted_card {
            Ok(card) => {
                return Ok(card.into())
            }
            Err(e) => {
                tracing::info!("{:?}", &e);
                let closed = self.clone().close_lithic_card(&token).await;
                tracing::info!("Closed card");
                match closed {
                    Ok(card) => {
                        tracing::info!("Rolled back lithic card successfully");
                    },
                    Err(err) => {
                        tracing::error!("Unable to close lithic card");
                        return Err(err);
                    }
                }
                return Err(PassthroughCardError::Unexpected("unable to issue card".into()));
            }
        }
    }

    // really lets rewrite this to be atomic
    #[tracing::instrument(skip(self))]
    async fn update_card_status(
        self: Arc<Self>,
        user: &User,
        status: PassthroughCardStatus
    ) -> Result<(), PassthroughCardError> {
        tracing::info!("Searching for cards for userId={} to go to status={}", user.id, &status);
        let card = self.clone().find_card_for_user_in_status(
            &user,
            &status
        ).await?;
        let previous_status = card.passthrough_card_status;
        tracing::info!("Found card={} for userId={}", card.id, user.id);

        let updated = self.passthrough_card_dao.clone().update_status(
            card.id,
            status
        ).await?;

        tracing::info!("Updated card={} for userId={}", card.id, user.id);

        let lithic_result = match &status {
            PassthroughCardStatus::Closed => self.clone().close_lithic_card(&updated.token).await,
            PassthroughCardStatus::Open => self.clone().activate_lithic_card(&updated.token).await,
            PassthroughCardStatus::Paused =>  self.clone().pause_lithic_card(&updated.token).await,
            _ => Err(PassthroughCardError::Unexpected("Invalid state transition from engine".into()))
        };

        return match lithic_result {
            Ok(card) => {
                tracing::info!("Successfully updated lithic status for cardId={} token={}", updated.id, updated.token);
                Ok(())
            },
            Err(e) => {
                // we really want to rollback here
                // will figure out later. for now logs
                tracing::error!("Error applying status update to lithic card for cardId={} token={}", updated.id, updated.token);
                // TODO: don't call direct
                let rollback = self.passthrough_card_dao.clone().update_status(
                    updated.id,
                    previous_status
                ).await;

                match rollback {
                    Ok(card) => {
                        tracing::info!("Rolled back internal status successfully");
                    },
                    Err(e) => {
                        tracing::error!("Error rolling back internal status");
                    }
                }
                Err(e)
            }
        };
        //Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn get_by_token(self: Arc<Self>, token: &str) -> Result<PassthroughCardModel, PassthroughCardError> {
        tracing::warn!("runtime: {:?}, task: {:?}", tokio::runtime::Handle::current().id(), tokio::task::id());
        Ok(self.passthrough_card_dao.clone().get_by_token(token).await?.into())
    }

}

impl PassthroughCardService {
    #[tracing::instrument(skip_all)]
    pub fn new() -> Self {
        Self {
            lithic_service: Arc::new(LithicService::new()),
            passthrough_card_dao: Arc::new(PassthroughCardDao::new())
        }
    }
    #[tracing::instrument(skip_all)]
    pub fn new_with_services(
        lithic_service: Arc<dyn LithicServiceTrait>,
    ) -> Self {
        Self {
            lithic_service,
            passthrough_card_dao: Arc::new(PassthroughCardDao::new())
        }
    }


    #[tracing::instrument(skip(self))]
    pub(super) async fn find_card_for_user_in_status(
        self: Arc<Self>,
        user: &User,
        status: &PassthroughCardStatus
    ) -> Result<PassthroughCard, PassthroughCardError> {
        // TODO: don't call db direct
        let cards: Vec<PassthroughCard> = self.passthrough_card_dao.clone().find_cards_for_user(user.id).await?;
        return match status {
            PassthroughCardStatus::Closed => {
                self.filter_cards(
                    &cards,
                    |card| {card.is_active.is_some_and(|active|active)}
                ).cloned()
            },
            PassthroughCardStatus::Open => {
                self.filter_cards(
                    &cards,
                    |item| {
                        item.is_active.is_some_and(|active| active)
                            && item.passthrough_card_status == PassthroughCardStatus::Paused
                    }
                ).cloned()
            },
            PassthroughCardStatus::Paused => {
                self.filter_cards(
                    &cards,
                    |item| {
                        item.is_active.is_some_and(|active| active)
                            && item.passthrough_card_status == PassthroughCardStatus::Open
                    }
                ).cloned()
            },
            _ => return Err(PassthroughCardError::StatusUpdate("Invalid state transition from engine".into()))
        }
    }

    #[tracing::instrument(skip(self))]
    pub(super) async fn get_active_card_for_user(
        self: Arc<Self>,
        user: &User
    ) -> Result<Option<PassthroughCard>, PassthroughCardError> {
        let cards: Vec<PassthroughCard> = self.passthrough_card_dao.clone().find_cards_for_user(user.id).await?;
        if cards.len() == 0 {
            return Ok(None);
        }
        let result: Vec<&PassthroughCard> = cards
            .iter()
            .filter(|&card| {
                return card.passthrough_card_status == PassthroughCardStatus::Open ||
                    card.passthrough_card_status == PassthroughCardStatus::Paused

            })
            .collect();
        if result.len() > 0 {
            if let Some(card) = result.get(0) {
                return Ok(Some((**card).clone()))
            }
            return Ok(None);
        }
        Ok(None)
    }

    #[tracing::instrument(skip(self))]
    pub(super) async fn user_has_active_card(
        self: Arc<Self>,
        user: &User
    ) -> Result<bool, PassthroughCardError> {
        if let Some(card) = self.clone().get_active_card_for_user(&user).await? {
            return Ok(true)
        }
        Ok(false)
    }

    //probably need a lifetime
    fn filter_cards<'a>(
        &'a self,
        cards: &'a Vec<PassthroughCard>,
        filter: fn(&PassthroughCard) -> bool
    ) -> Result<&PassthroughCard, PassthroughCardError> {
        let v: Vec<&PassthroughCard> = cards
            .iter()
            .filter(|item| filter(item))
            //.cloned()
            .collect();
        // TODO: this scares me
        Ok(v.get(0).ok_or(
            PassthroughCardError::CardNotFound("card to transition not found".into())
        )?)
    }

    #[tracing::instrument(skip(self))]
    async fn close_lithic_card(
        self: Arc<Self>,
        token: &str
    ) -> Result<Card, PassthroughCardError> {
        let closed = self.lithic_service.clone().close_card(token)
            .await.map_err(|e| PassthroughCardError::StatusUpdate(Box::new(e)))?;
        Ok(closed)
    }

    async fn pause_lithic_card(
        self: Arc<Self>,
        token: &str
    ) -> Result<Card, PassthroughCardError> {
        let paused = self.lithic_service.clone().pause_card(token)
            .await.map_err(|e| PassthroughCardError::StatusUpdate(Box::new(e)))?;
        Ok(paused)
    }

    async fn activate_lithic_card(
        self: Arc<Self>,
        token: &str
    ) -> Result<Card, PassthroughCardError> {
        let active = self.lithic_service.clone().activate_card(token)
            .await.map_err(|e| PassthroughCardError::StatusUpdate(Box::new(e)))?;
        Ok(active)
    }
}