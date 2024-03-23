use lithic_client::models::Card;
use uuid::Uuid;
use crate::passthrough_card::constant::{PassthroughCardStatus, PassthroughCardType};
use crate::passthrough_card::entity::InsertablePassthroughCard;
use crate::passthrough_card::error::PassthroughCardError;
use crate::passthrough_card::request::LithicCard;
use crate::user::model::UserModel;
use crate::util::date::expiration_date_from_str_parts;

impl TryFrom<LithicCard> for InsertablePassthroughCard {
    type Error = PassthroughCardError;

    fn try_from(card: LithicCard) -> Result<Self, Self::Error> {
        Ok(InsertablePassthroughCard {
            passthrough_card_status: PassthroughCardStatus::Open,
            public_id: Uuid::new_v4(),
            user_id: 0,
            token: card.token.to_string(),
            expiration: expiration_date_from_str_parts(&card.exp_year, &card.exp_month)
                .map_err(|e| PassthroughCardError::Unexpected(e.into()))?,
            last_four: card.last_four.to_string(),
            passthrough_card_type: PassthroughCardType::Virtual,
            is_active: true
        })
    }
}

impl TryFrom<(Card, &UserModel)> for InsertablePassthroughCard {
    type Error = PassthroughCardError;

    fn try_from(value: (Card, &UserModel)) -> Result<Self, Self::Error> {
        let card = value.0;
        let user = value.1;
        let exp_year = card.exp_year.clone().ok_or(
            PassthroughCardError::Unexpected("Cannot find expiration year".into())
        )?;
        let exp_month = card.exp_month.clone().ok_or(
            PassthroughCardError::Unexpected("Cannot find expiration month".into())
        )?;
        let expiration = expiration_date_from_str_parts(&exp_year, &exp_month)
            .map_err(|e| PassthroughCardError::Unexpected(e.into()))?;
        Ok(InsertablePassthroughCard {
            passthrough_card_status: PassthroughCardStatus::Open,
            public_id: Uuid::new_v4(),
            user_id: user.id,
            token: card.token.to_string(),
            expiration: expiration,
            last_four: card.last_four.clone(),
            passthrough_card_type: PassthroughCardType::Virtual,
            is_active: true
        })
    }
}