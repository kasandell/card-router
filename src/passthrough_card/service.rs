use std::sync::Arc;
use async_trait::async_trait;
use base64::Engine as base64Engine;
use uuid::Uuid;
use crate::user::entity::User;
use crate::service_error::ServiceError;
use crate::passthrough_card::constant::PassthroughCardStatus;
use crate::passthrough_card::entity::PassthroughCard;
use base64::engine::general_purpose;
use lithic_client::models::card::Card;
use crate::error_type::ErrorType;
use crate::lithic::{
    service::LithicService,
    service::LithicServiceTrait,
};
use crate::passthrough_card::crypto::encrypt_pin;

pub struct PassthroughCardService {
    pub lithic_service: Arc<dyn LithicServiceTrait>
}

impl PassthroughCardService {
    pub fn new() -> Self {
        Self {
            lithic_service: Arc::new(LithicService::new())
        }
    }
    pub fn new_with_service(service: Arc<dyn LithicServiceTrait>) -> Self {
        Self {
            lithic_service: service
        }
    }
    pub async fn issue_card_to_user(
        self: Arc<Self>,
        user: &User,
        pin: &str
    ) -> Result<PassthroughCard, ServiceError> {
        let has_active = self.clone().user_has_active_card(&user).await?;
        if has_active {
            return Err(ServiceError::new(ErrorType::Conflict, "User has active card already"))
        }
        let idempotency_key = Uuid::new_v4();
        let pin_encoded = encrypt_pin(pin);
        let lithic_card = self.lithic_service.clone().create_card(
            &pin_encoded,
            &idempotency_key
        ).await?;
        let inserted_card = PassthroughCard::create_from_api_card(
            &lithic_card,
            &user
        ).await;
        return match inserted_card {
            Ok(card) => {
                return Ok(card)
            }
            Err(e) => {
                println!("{}", e.message);
                let closed = self.clone().close_lithic_card(&lithic_card.token.to_string()).await;
                println!("Closed card");
                match closed {
                    Ok(card) => {
                        info!("Rolled back lithic card successfully");
                        println!("Rolled back lithic card successfully");
                    },
                    Err(err) => {
                        error!("Unable to close lithic card");
                        println!("Unable to close lithic card");
                        println!("{}", err.message);
                        return Err(err);
                    }
                }
                return Err(ServiceError::new(ErrorType::InternalServerError, "unable to issue card"));
            }
        }
    }

    // really lets rewrite this to be atomic
    pub async fn update_card_status(
        self: Arc<Self>,
        user: &User,
        status: PassthroughCardStatus
    ) -> Result<(), ServiceError> {
        info!("Searching for cards for userId={} to go to status={}", user.id, &status);
        let card = self.clone().find_card_for_user_in_status(
            &user,
            &status
        ).await?;
        let previous_status = card.passthrough_card_status;
        info!("Found card={} for userId={}", card.id, user.id);

        let updated = PassthroughCard::update_status(
            card.id,
            status
        ).await?;

        info!("Updated card={} for userId={}", card.id, user.id);

        let lithic_result = match &status {
            PassthroughCardStatus::Closed => self.close_lithic_card(&updated.token).await,
            PassthroughCardStatus::Open => self.activate_lithic_card(&updated.token).await,
            PassthroughCardStatus::Paused =>  self.pause_lithic_card(&updated.token).await,
            _ => Err(ServiceError::new(ErrorType::InternalServerError, "Invalid state transition from engine"))
        };

        return match lithic_result {
            Ok(card) => {
                info!("Successfully updated lithic status for cardId={} token={}", updated.id, updated.token);
                println!("Successfully updated lithic status for cardId={} token={}", updated.id, updated.token);
                Ok(())
            },
            Err(e) => {
                // we really want to rollback here
                // will figure out later. for now logs
                error!("Error applying status update to lithic card for cardId={} token={}", updated.id, updated.token);
                println!("Error applying status update to lithic card for cardId={} token={}", updated.id, updated.token);
                // TODO: don't call direct
                let rollback = PassthroughCard::update_status(
                    updated.id,
                    previous_status
                ).await;

                match rollback {
                    Ok(card) => {
                        info!("Rolled back internal status successfully");
                        println!("Rolled back internal status successfully");

                    },
                    Err(e) => {
                        error!("Error rolling back internal status");
                        println!("Error rolling back internal status");
                    }
                }
                Err(e)
            }
        };
        //Ok(())
    }

    pub async fn find_card_for_user_in_status(
        self: Arc<Self>,
        user: &User,
        status: &PassthroughCardStatus
    ) -> Result<PassthroughCard, ServiceError> {
        // TODO: don't call db direct
        let cards: Vec<PassthroughCard> = PassthroughCard::find_cards_for_user(user.id).await?;
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
            _ => return Err(ServiceError::new(ErrorType::NotFound, "Invalid state transition from engine"))
        }
    }

    pub async fn get_active_card_for_user(
        self: Arc<Self>,
        user: &User
    ) -> Result<Option<PassthroughCard>, ServiceError> {
        let cards = PassthroughCard::find_cards_for_user(user.id).await?;
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

    pub async fn user_has_active_card(
        self: Arc<Self>,
        user: &User
    ) -> Result<bool, ServiceError> {
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
    ) -> Result<&PassthroughCard, ServiceError> {
        let v: Vec<&PassthroughCard> = cards
            .iter()
            .filter(|item| filter(item))
            //.cloned()
            .collect();
        // TODO: this scares me
        Ok(v.get(0).ok_or(
            ServiceError::new(ErrorType::NotFound, "card to transition not found")
        )?)
    }

    async fn close_lithic_card(
        self: Arc<Self>,
        token: &str
    ) -> Result<Card, ServiceError> {
        let closed = self.lithic_service.clone().close_card(token).await?;
        Ok(closed)
    }

    async fn pause_lithic_card(
        self: Arc<Self>,
        token: &str
    ) -> Result<Card, ServiceError> {
        let closed = self.lithic_service.clone().pause_card(token).await?;
        Ok(closed)
    }

    async fn activate_lithic_card(
        self: Arc<Self>,
        token: &str
    ) -> Result<Card, ServiceError> {
        let closed = self.lithic_service.clone().activate_card(token).await?;
        Ok(closed)
    }
}