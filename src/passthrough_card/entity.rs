use crate::schema::passthrough_card;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::user::entity::User;
use super::constant::{
    PassThroughCardStatus,
    PassthroughCardType
};

pub struct LithicCard {
    pub token: String,
    pub last_four: String,
    pub exp_month: String,
    pub exp_year: String
}


#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Identifiable)]
#[diesel(table_name = passthrough_card)]
#[diesel(belongs_to(PassthroughCardType))]
#[diesel(belongs_to(PassThroughCardStatus))]
pub struct PassthroughCard {
    pub id: i32,
    pub public_id: Uuid,
    pub passthrough_card_status_id: i32,
    pub is_active: Optional<Bool>,
    pub user_id: i32,
    pub token: String,
    pub expiration: Date,
    pub last_four: String,
    pub passthrough_card_type_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = passthrough_card)]
#[diesel(belongs_to(PassthroughCardType))]
#[diesel(belongs_to(PassThroughCardStatus))]
pub struct InsertablePassthroughCard {
    pub passthrough_card_status_id: i32,
    pub public_id: Uuid,
    pub user_id: i32,
    pub token: String,
    pub expiration: Date,
    pub last_four: String,
    pub passthrough_card_type_id: i32,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = passthrough_card)]
#[diesel(belongs_to(PassThroughCardStatus))]
pub struct PassthroughCardStatusUpdate {
    pub id: i32,
    pub passthrough_card_status_id: i32,
}

impl PassthroughCard {
    pub fn create(card: LithicCard/*, user: User */) -> Result<Self, ApiError> {
        let mut conn = db::connection()?;

        let card = InsertablePassthroughCard::from(card);
        //TODO: populate with user_id
        //card.user_id = user.id;
        let card = diesel::insert_into(passthrough_card::table)
            .values(card)
            .get_result(&mut conn)?;
        Ok(card)
    }

    pub fn update_status(id: i32, status: PassthroughCardStatus) {
        let mut conn = db::connection()?;
        let update = PassthroughCardStatusUpdate {
            id: id,
            status: status.as_str()
        };
        let update = diesel::update(passthrough_card::table)
        .filter(id::id.eq(id))
        .set(update)
        .get_result(&mut conn)?;
        Ok(update)
    }
}

impl From<LithicCard> for InsertablePassthroughCard {
    fn from(card: LithicCard) -> Self {
        InsertablePassthroughCard {
            passthorugh_card_type: 1,
            public_id: Uuid::new_v4(),
            user_id: 0,
            token: card.token,
            expiration: Date::now(), // todo: from month and year,
            last_four: card.last_four
        }
    }
}