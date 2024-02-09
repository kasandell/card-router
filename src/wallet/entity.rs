use crate::{schema::{
    wallet,
    wallet_card_attempt,
    credit_card,
    credit_card_issuer,
    credit_card_type
}, credit_card_type::entity::{CreditCardType,CreditCard,CreditCardIssuer}};
use crate::util::db;
use crate::api_error::ApiError;
use crate::user::entity::User;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug, Selectable, Clone)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(CreditCard))]
#[diesel(table_name = wallet_card_attempt)]
pub struct WalletCardAttempt {
    pub id: i32,
    pub public_id: Uuid,
    pub user_id: i32,
    pub credit_card_id: i32,
    pub expected_reference_id: String,
    pub psp_id: Option<String>,
    pub status: String,
    pub recurring_detail_reference: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Clone)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(CreditCard))]
#[diesel(table_name = wallet_card_attempt)]
pub struct InsertableCardAttempt {
    pub user_id: i32,
    pub credit_card_id: i32,
    pub expected_reference_id: String,
}

#[derive(Serialize, Deserialize, AsChangeset, Clone)]
#[diesel(table_name = wallet_card_attempt)]
pub struct UpdateCardAttempt {
    pub recurring_detail_reference: String,
    pub psp_id: String,
    pub status: String
}


#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug, Selectable, Clone)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(CreditCard))]
#[diesel(table_name = wallet)]
pub struct Wallet {
    pub id: i32,
    pub public_id: Uuid,
    pub user_id: i32,
    pub payment_method_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub credit_card_id: i32,
    pub wallet_card_attempt_id: i32,
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(CreditCard))]
#[diesel(table_name = wallet)]
pub struct InsertableCard {
    pub public_id: Uuid,
    pub user_id: i32,
    pub payment_method_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub credit_card_id: i32,
    pub wallet_card_attempt_id: i32,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(CreditCard))]
#[diesel(table_name = wallet)]
pub struct NewCard {
    pub user_id: i32,
    pub payment_method_id: String,
    pub credit_card_id: i32,
    pub wallet_card_attempt_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisplayableCardInfo {
    pub public_id: Uuid,
    pub created_at: NaiveDateTime,
    pub card_name: String,
    pub issuer_name: String,
    pub card_type: String,
    pub card_image_url: String,
}

impl Wallet {
    pub fn find_all_for_user(user: &User) -> Result<Vec<Self>, ApiError> {
        let mut conn = db::connection()?;
        //let cards = Wallet::belonging_to(&user).load::<Wallet>(&mut conn)?;
        let cards = wallet::table.filter(
            wallet::user_id.eq(user.id)
        ).load::<Wallet>(&mut conn)?;
        Ok(cards) 
    }

    /**/
    pub fn find_all_for_user_with_card_info(user: &User) -> Result<Vec<(Self, CreditCard, CreditCardType, CreditCardIssuer)>, ApiError> {
        let mut conn = db::connection()?;
        let cards = wallet::table
        .inner_join(
            credit_card::table
                .inner_join(credit_card_issuer::table)
                .inner_join(credit_card_type::table)
        )
        .filter(
            wallet::user_id.eq(user.id)
        )
        .select((Wallet::as_select(), CreditCard::as_select(), CreditCardType::as_select(), CreditCardIssuer::as_select()))
        .load::<(Wallet, CreditCard, CreditCardType, CreditCardIssuer)>(&mut conn)?;
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

#[cfg(test)]
pub fn delete(id: i32) -> Result<usize, ApiError> {
    let mut conn = db::connection()?;

    let res = diesel::delete(
            wallet::table
                .filter(wallet::id.eq(id))
        )
        .execute(&mut conn)?;
    Ok(res)
}

#[cfg(test)]
pub fn delete_self(&self) -> Result<usize, ApiError> {
    Wallet::delete(self.id)
}
}

impl From<NewCard> for InsertableCard {
fn from(card: NewCard) -> Self {
    InsertableCard {
        public_id: Uuid::new_v4(),
        user_id: card.user_id,
        payment_method_id: card.payment_method_id,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        credit_card_id: card.credit_card_id,
        wallet_card_attempt_id: card.wallet_card_attempt_id
    }
}
}

#[cfg(test)]
impl Wallet {
pub fn create_test_wallet(
    id: i32,
    user_id: i32,
    credit_card_id: i32
) -> Self {
    Wallet {
        id: id,
        public_id: Uuid::new_v4(),
        user_id: user_id,
        payment_method_id: Uuid::new_v4().to_string(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        credit_card_id: credit_card_id,
        wallet_card_attempt_id: 0
    }
}
}

impl WalletCardAttempt {
pub fn insert(card_attempt: InsertableCardAttempt) -> Result<Self, ApiError> {
    let mut conn = db::connection()?;

    let wallet = diesel::insert_into(wallet_card_attempt::table)
    .values(card_attempt)
    .get_result::<WalletCardAttempt>(&mut conn)?;
    Ok(wallet)
}

pub fn find_by_reference_id(reference: String) -> Result<Self, ApiError> {
    let mut conn = db::connection()?;

    let card_attempt = wallet_card_attempt::table
        .filter(wallet_card_attempt::expected_reference_id.eq(reference))
        .first(&mut conn)?;

    Ok(card_attempt)
}

pub fn update_card(id: i32, card: UpdateCardAttempt) -> Result<Self, ApiError> {
    let mut conn = db::connection()?;

    let wallet = diesel::update(wallet_card_attempt::table)
        .filter(wallet_card_attempt::id.eq(id))
        .set(card)
        .get_result::<WalletCardAttempt>(&mut conn)?;
    Ok(wallet)
}

#[cfg(test)]
pub fn delete(id: i32) -> Result<usize, ApiError> {
    let mut conn = db::connection()?;

    let res = diesel::delete(
        wallet_card_attempt::table
            .filter(wallet_card_attempt::id.eq(id))
    )
        .execute(&mut conn)?;
    Ok(res)
}

#[cfg(test)]
pub fn delete_self(&self) -> Result<usize, ApiError> {
    WalletCardAttempt::delete(self.id)
}
}