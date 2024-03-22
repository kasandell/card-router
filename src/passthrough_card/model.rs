use chrono::{NaiveDate, NaiveDateTime};
use uuid::Uuid;
use crate::passthrough_card::constant::{PassthroughCardStatus, PassthroughCardType};
use crate::passthrough_card::entity::PassthroughCard;

#[derive(Clone, Debug)]
pub struct PassthroughCardModel {
    pub id: i32,
    pub public_id: Uuid,
    pub passthrough_card_status: PassthroughCardStatus,
    pub is_active: Option<bool>,
    pub user_id: i32,
    pub token: String,
    pub last_four: String,
    pub expiration: NaiveDate,
    pub passthrough_card_type: PassthroughCardType,
    pub created_at: NaiveDateTime,
}

impl From<PassthroughCard> for PassthroughCardModel {
    fn from(value: PassthroughCard) -> Self {
        PassthroughCardModel {
            id: value.id,
            public_id: value.public_id,
            passthrough_card_status: value.passthrough_card_status,
            is_active: value.is_active,
            user_id: value.user_id,
            token: value.token,
            last_four: value.last_four,
            expiration: value.expiration,
            passthrough_card_type: value.passthrough_card_type,
            created_at: value.created_at
        }
    }
}