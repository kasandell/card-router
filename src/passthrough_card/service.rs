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
        tracing::info!("Issuing card to user_id={}", user.id);
        let has_active = self.clone().user_has_active_card(&user).await?;
        if has_active {
            tracing::warn!("User has active card already");
            return Err(PassthroughCardError::ActiveCardExists("User has active card already".into()))
        }
        let idempotency_key = Uuid::new_v4();
        let pin_encoded = encrypt_pin(pin);
        let lithic_card = self.lithic_service.clone().create_card(
            &pin_encoded,
            &idempotency_key
        ).await.map_err(|e| {
            tracing::error!("Error issuing card in lithic call");
            PassthroughCardError::IssueCard(e.into())
        })?;
        let token = lithic_card.token.to_string();
        tracing::info!("Mapping lithic to internal card");
        let card = InsertablePassthroughCard::try_from((lithic_card, user))?;
        tracing::info!("Inserting card");
        let inserted_card = self.passthrough_card_dao.clone().create(
            card
        ).await;
        return match inserted_card {
            Ok(card) => {
                tracing::info!("Succesffully inserted card");
                return Ok(card.into())
            }
            Err(e) => {
                tracing::error!("Error inserting card {:?}", &e);
                tracing::info!("Rolling back lithic card creation");
                let closed = self.clone().close_lithic_card(&token).await;
                match closed {
                    Ok(card) => {
                        tracing::info!("Rolled back lithic card successfully");
                    },
                    Err(err) => {
                        tracing::error!("Unable to close lithic card error={:?}", &err);
                        return Err(PassthroughCardError::Unexpected(err.into()));
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

        tracing::info!("Transitioning card in lithic");
        let lithic_result = match &status {
            PassthroughCardStatus::Closed => self.clone().close_lithic_card(&updated.token).await,
            PassthroughCardStatus::Open => self.clone().activate_lithic_card(&updated.token).await,
            PassthroughCardStatus::Paused =>  self.clone().pause_lithic_card(&updated.token).await,
            _ => {
                tracing::error!("Invalid state transition state={}", &status);
                Err(PassthroughCardError::Unexpected("Invalid state transition from engine".into()))
            }
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
                        // TODO: Error out of this branch?
                    }
                }
                Err(e)
            }
        };
        //Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn get_by_token(self: Arc<Self>, token: &str) -> Result<PassthroughCardModel, PassthroughCardError> {
        tracing::info!("Getting card by token={}", token);
        Ok(self.passthrough_card_dao.clone().get_by_token(token).await?.into())
    }

}

impl PassthroughCardService {
    #[cfg_attr(feature="trace-detail", tracing::instrument(skip_all))]
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
        tracing::info!("Finding cards for user_id={} in status={:?}", user.id, status);
        let cards: Vec<PassthroughCard> = self.passthrough_card_dao.clone().find_cards_for_user(user.id).await?;
        tracing::info!("Found {} cards to filter", cards.len());
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
            _ => {
                tracing::error!("Invalid state transition found in passthrough card state={:?}", &status);
                return Err(PassthroughCardError::StatusUpdate("Invalid state transition from engine".into()))
            }
        }
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    pub(super) async fn get_active_card_for_user(
        self: Arc<Self>,
        user: &User
    ) -> Result<Option<PassthroughCard>, PassthroughCardError> {
        tracing::info!("Getting active card for user_id={}", user.id);
        let cards: Vec<PassthroughCard> = self.passthrough_card_dao.clone().find_cards_for_user(user.id).await?;
        if cards.len() == 0 {
            tracing::info!("No cards for user found in any state");
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
            tracing::info!("Found {} cards for user", result.len());
            if let Some(card) = result.get(0) {
                tracing::info!("Found card in state={:?}", &card.passthrough_card_status);
                return Ok(Some((**card).clone()))
            }
            tracing::warn!("Unable to get card in list");
            return Ok(None);
        }
        tracing::warn!("No active cards found for user");
        Ok(None)
    }

    #[tracing::instrument(skip(self))]
    pub(super) async fn user_has_active_card(
        self: Arc<Self>,
        user: &User
    ) -> Result<bool, PassthroughCardError> {
        tracing::info!("Checking if user has active card");
        if let Some(card) = self.clone().get_active_card_for_user(&user).await? {
            return Ok(true)
        }
        Ok(false)
    }

    //probably need a lifetime
    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
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
        Ok(v.get(0).ok_or_else(||
            {
                tracing::error!("No card found to transition");
                return PassthroughCardError::CardNotFound("card to transition not found".into())
            }
        )?)
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn close_lithic_card(
        self: Arc<Self>,
        token: &str
    ) -> Result<Card, PassthroughCardError> {
        tracing::info!("Closing card");
        let closed = self.lithic_service.clone().close_card(token)
            .await.map_err(|e| PassthroughCardError::StatusUpdate(Box::new(e)))?;
        Ok(closed)
    }


    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn pause_lithic_card(
        self: Arc<Self>,
        token: &str
    ) -> Result<Card, PassthroughCardError> {
        tracing::info!("Pausing card");
        let paused = self.lithic_service.clone().pause_card(token)
            .await.map_err(|e| PassthroughCardError::StatusUpdate(Box::new(e)))?;
        Ok(paused)
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn activate_lithic_card(
        self: Arc<Self>,
        token: &str
    ) -> Result<Card, PassthroughCardError> {
        tracing::info!("Activating card");
        let active = self.lithic_service.clone().activate_card(token)
            .await.map_err(|e| PassthroughCardError::StatusUpdate(Box::new(e)))?;
        Ok(active)
    }
}

#[cfg(test)]
impl PassthroughCardService {
    pub fn new_with_mocks(
        lithic_service: Arc<dyn LithicServiceTrait>,
        passthrough_card_dao: Arc<dyn PassthroughCardDaoTrait>
    ) -> Self {
        Self {
            lithic_service,
            passthrough_card_dao
        }
    }
}