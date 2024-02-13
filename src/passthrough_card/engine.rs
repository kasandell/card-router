use base64::Engine as base64Engine;
use uuid::Uuid;
use crate::user::entity::User;
use crate::api_error::ApiError;
use crate::passthrough_card::constant::PassthroughCardStatus;
use crate::passthrough_card::entity::PassthroughCard;
use base64::engine::general_purpose;
use lithic_client::models::card::Card;
use crate::lithic_service::{
    service::LithicService,
    service::LithicServiceTrait,
};

pub struct Engine {
    pub lithic_service: Box<dyn LithicServiceTrait>
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            lithic_service: Box::new(LithicService::new())
        }
    }
    #[cfg(test)]
    pub fn new_with_service(service: Box<dyn LithicServiceTrait>) -> Self {
        Engine {
            lithic_service: service
        }
    }
    pub fn issue_card_to_user(
        &self,
        user: &User,
        pin: String
    ) -> Result<PassthroughCard, ApiError> {
        let has_active = self.user_has_active_card(&user)?;
        if has_active {
            return Err(ApiError::new(409, "User has active card already".to_string()))
        }
        let idempotency_key = Uuid::new_v4();
        let pin_encoded = general_purpose::STANDARD_NO_PAD.encode(pin);
        let lithic_card = self.lithic_service.create_card(
            pin_encoded,
            idempotency_key
        )?;
        let inserted_card = PassthroughCard::create_from_api_card(
            &lithic_card,
            &user
        );
        return match inserted_card {
            Ok(card) => {
                return Ok(card)
            }
            Err(e) => {
                let closed = self.close_lithic_card(&lithic_card.token.to_string());
                match closed {
                    Ok(card) => info!("Rolled back lithic card successfully"),
                    Err(err) => {
                        error!("Unable to close lithic card");
                        return Err(err);
                    }
                }
                return Err(ApiError::new(500, "unable to issue card".to_string()));
            }
        }
    }

    // really lets rewrite this to be atomic
    pub fn update_card_status(
        &self,
        user: &User,
        status: PassthroughCardStatus
    ) -> Result<(), ApiError> {
        info!("Searching for cards for userId={} to go to status={}", user.id, String::from(&status));
        let card = self.find_card_for_user_in_status(
            &user,
            &status
        )?;
        let previous_status = card.passthrough_card_status;
        info!("Found card={} for userId={}", card.id, user.id);

        let updated = PassthroughCard::update_status(
            card.id,
            status.clone()
        )?;

        info!("Updated card={} for userId={}", card.id, user.id);

        /*
        let lithic_result = match &status {
            PassthroughCardStatus::CLOSED => self.close_lithic_card(&updated.token),
            PassthroughCardStatus::OPEN => self.activate_lithic_card(&updated.token),
            PassthroughCardStatus::PAUSED =>  self.pause_lithic_card(&updated.token),
            _ => Err(ApiError::new(500, "Invalid state transition from engine".to_string()))
        };

        return match lithic_result {
            Ok(card) => {
                info!("Successfully updated lithic status for cardId={} token={}", updated.id, updated.token);
                Ok(())
            },
            Err(e) => {
                // we really want to rollback here
                // will figure out later. for now logs
                error!("Error applying status update to lithic card for cardId={} token={}", updated.id, updated.token);
                let rollback = PassthroughCard::update_status(
                    updated.id,
                    PassthroughCardStatus::from(&*previous_status)
                );

                match rollback {
                    Ok(card) => info!("Rolled back internal status successfully"),
                    Err(e) => error!("Error rolling back internal status")
                }
                Err(e)
            }
        };
         */
        Ok(())
    }

    pub fn find_card_for_user_in_status(
        &self,
        user: &User,
        status: &PassthroughCardStatus
    ) -> Result<PassthroughCard, ApiError> {
        return match status {
            PassthroughCardStatus::CLOSED => {
                let v: Vec<PassthroughCard> = PassthroughCard::find_cards_for_user(user.id)?;
                self.filter_cards(
                    &v,
                    |card| {card.is_active.is_some_and(|active|active)}
                ).cloned()
            },
            PassthroughCardStatus::OPEN => {
                let v: Vec<PassthroughCard> = PassthroughCard::find_cards_for_user(user.id)?;
                self.filter_cards(
                    &v,
                    |item| {
                        item.is_active.is_some_and(|active| active)
                            && item.passthrough_card_status == String::from(&PassthroughCardStatus::PAUSED)
                    }
                ).cloned()
            },
            PassthroughCardStatus::PAUSED => {
                let v: Vec<PassthroughCard> = PassthroughCard::find_cards_for_user(user.id)?;
                self.filter_cards(
                    &v,
                    |item| {
                        item.is_active.is_some_and(|active| active)
                            && item.passthrough_card_status == String::from(&PassthroughCardStatus::OPEN)
                    }
                ).cloned()
            },
            _ => return Err(ApiError::new(404, "Invalid state transition from engine".to_string()))
        }
    }

    pub fn get_active_card_for_user(
        &self,
        user: &User
    ) -> Result<Option<PassthroughCard>, ApiError> {
        let cards = PassthroughCard::find_cards_for_user(user.id)?;
        if cards.len() == 0 {
            return Ok(None);
        }
        let result: Vec<&PassthroughCard> = cards
            .iter()
            .filter(|&card| {
                return card.passthrough_card_status == String::from(&PassthroughCardStatus::OPEN) ||
                    card.passthrough_card_status == String::from(&PassthroughCardStatus::PAUSED)

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

    pub fn user_has_active_card(
        &self,
        user: &User
    ) -> Result<bool, ApiError> {
        if let Some(card) = self.get_active_card_for_user(&user)? {
            return Ok(true)
        }
        Ok(false)
        /*
        let cards = PassthroughCard::find_cards_for_user(user.id)?;
        if cards.len() == 0 {
            return Ok(false);
        }
        let result: Vec<&PassthroughCard> = cards
            .iter()
            .filter(|&card| {
                return card.passthrough_card_status == String::from(&PassthroughCardStatus::OPEN) ||
                    card.passthrough_card_status == String::from(&PassthroughCardStatus::PAUSED)

            })
            .collect();
        if result.len() > 0 {
            return Ok(true);
        }
        Ok(false)
         */
    }

    //probably need a lifetime
    fn filter_cards<'a>(
        &'a self,
        cards: &'a Vec<PassthroughCard>,
        filter: fn(&PassthroughCard) -> bool
    ) -> Result<&PassthroughCard, ApiError> {
        let v: Vec<&PassthroughCard> = cards
            .iter()
            .filter(|item| filter(item))
            //.cloned()
            .collect();
        // TODO: this scares me
        Ok(v.get(0).ok_or(
            ApiError::new(404, "card to transition not found".to_string())
        )?)
    }

    fn close_lithic_card(
        &self,
        token: &str
    ) -> Result<Card, ApiError> {
        let closed = self.lithic_service.close_card(token.to_string())?;
        Ok(closed)
    }

    fn pause_lithic_card(
        &self,
        token: &str
    ) -> Result<Card, ApiError> {
        let closed = self.lithic_service.pause_card(token.to_string())?;
        Ok(closed)
    }

    fn activate_lithic_card(
        &self,
        token: &str
    ) -> Result<Card, ApiError> {
        let closed = self.lithic_service.activate_card(token.to_string())?;
        Ok(closed)
    }
}