use crate::schema::wallet;
use crate::util::db;
use crate::api_error::ApiError;
use crate::user::entity::User;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug)]
#[diesel(belongs_to(User))]
#[diesel(table_name = wallet)]
pub struct Wallet {
    pub id: i32,
    pub public_id: Uuid,
    pub user_id: i32,
    pub stripe_payment_method_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[diesel(belongs_to(User))]
#[diesel(table_name = wallet)]
pub struct InsertableCard {
    pub public_id: Uuid,
    pub user_id: i32,
    pub stripe_payment_method_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[diesel(belongs_to(User))]
#[diesel(table_name = wallet)]
pub struct NewCard {
    pub user_id: i32,
    pub stripe_payment_method_id: String
}

impl Wallet {
    pub fn find_all_for_user(user: User) -> Result<Vec<Self>, ApiError> {
        let mut conn = db::connection()?;
        //let cards = Wallet::belonging_to(&user).load::<Wallet>(&mut conn)?;
        let cards = wallet::table.filter(
            wallet::user_id.eq(user.id)
        ).load::<Wallet>(&mut conn)?;
        Ok(cards) 
    }

    pub fn insert_card(card: NewCard) -> Result<Self, ApiError> {
        let mut conn = db::connection()?;
        let insertable_card = InsertableCard::from(card);
        let inserted_card = diesel::insert_into(wallet::table)
        .values(insertable_card)
        .get_result(&mut conn)?;
        Ok(inserted_card)

    }
}

impl From<NewCard> for InsertableCard {
    fn from(card: NewCard) -> Self {
        InsertableCard {
            public_id: Uuid::new_v4(),
            user_id: card.user_id,
            stripe_payment_method_id: card.stripe_payment_method_id,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}