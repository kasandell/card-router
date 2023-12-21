use base64::Engine as base64Engine;
use uuid::Uuid;
use crate::user::entity::User;
use crate::api_error::ApiError;
use crate::passthrough_card::constant::PassthroughCardStatus;
use crate::passthrough_card::entity::{InsertablePassthroughCard, PassthroughCard};
use base64::engine::general_purpose;
use lithic_client::models::card::Card;
use lithic_client::models::payment_response::Source::Lithic;
use crate::lithic_service::{
    service::LithicService,
    error::Error as LithicError
};

pub struct Engine {}

impl Engine {
    pub fn issue_card_to_user(
        user: User,
        pin: String
    ) -> Result<(), ApiError> {
        let idempotency_key = Uuid::new_v4();
        let pin_encoded = general_purpose::STANDARD_NO_PAD.encode(pin);
        let lithic_card = LithicService::create_card(
            pin_encoded,
            idempotency_key
        )?;
        let inserted_card = PassthroughCard::create_from_api_card(
            &lithic_card,
            &user
        );
        return match inserted_card {
            Ok(card) => {
                return Ok(())
            }
            Err(e) => {
                let closed = Engine::close_lithic_card(&lithic_card.token.to_string());
                return Err(ApiError::new(500, "unable to issue card".to_string()));
            }
        }
    }

    // really lets rewrite this to be atomic
    pub fn update_card_status(
        user: User,
        status: PassthroughCardStatus
    ) -> Result<(), ApiError> {
        info!("Searching for cards for userId={} to go to status={}", user.id, String::from(&status));
        let card = Engine::find_card_for_user_in_status(
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

        let lithic_result = match &status {
            PassthroughCardStatus::CLOSED => Engine::close_lithic_card(&updated.token),
            PassthroughCardStatus::OPEN => Engine::activate_lithic_card(&updated.token),
            PassthroughCardStatus::PAUSED =>  Engine::activate_lithic_card(&updated.token),
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
    }

    pub fn find_card_for_user_in_status(
        user: &User,
        status: &PassthroughCardStatus
    ) -> Result<PassthroughCard, ApiError> {
        return match status {
            PassthroughCardStatus::CLOSED => {
                let v: Vec<PassthroughCard> = PassthroughCard::find_cards_for_user(user.id)?;
                Engine::filter_cards(
                    &v,
                    |card| {card.is_active.is_some_and(|active|active)}
                ).cloned()
            },
            PassthroughCardStatus::OPEN => {
                let v: Vec<PassthroughCard> = PassthroughCard::find_cards_for_user(user.id)?;
                Engine::filter_cards(
                    &v,
                    |item| {
                        item.is_active.is_some_and(|active| active)
                            && item.passthrough_card_status == String::from(&PassthroughCardStatus::PAUSED)
                    }
                ).cloned()
            },
            PassthroughCardStatus::PAUSED => {
                let v: Vec<PassthroughCard> = PassthroughCard::find_cards_for_user(user.id)?;
                Engine::filter_cards(
                    &v,
                    |item| {
                        item.is_active.is_some_and(|active| active)
                            && item.passthrough_card_status == String::from(&PassthroughCardStatus::OPEN)
                    }
                ).cloned()
            },
            _ => return Err(ApiError::new(500, "Invalid state transition from engine".to_string()))
        }
    }

    //probably need a lifetime
    fn filter_cards(
        cards: &Vec<PassthroughCard>,
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
        token: &str
    ) -> Result<Card, ApiError> {
        let closed = LithicService::close_card(token.to_string())?;
        Ok(closed)
    }

    fn pause_lithic_card(
        token: &str
    ) -> Result<Card, ApiError> {
        let closed = LithicService::pause_card(token.to_string())?;
        Ok(closed)
    }

    fn activate_lithic_card(
        token: &str
    ) -> Result<Card, ApiError> {
        let closed = LithicService::activate_card(token.to_string())?;
        Ok(closed)
    }
}