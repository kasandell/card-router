use crate::{credit_card_type::model::{
    CreditCardIssuerModel as CreditCardIssuer,
    CreditCardModel as CreditCard,
    CreditCardTypeModel as CreditCardType
}, schema::{
    credit_card,
    credit_card_issuer,
    credit_card_type,
    wallet,
    wallet_card_attempt
}};
use crate::util::db;
use crate::error::data_error::DataError;
use crate::user::model::UserModel as User;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use uuidv7;
use crate::wallet::constant::WalletCardAttemptStatus;

// TODO: this needs to be shortened down
pub type WalletDetail = (Wallet, CreditCard, CreditCardType, CreditCardIssuer);

#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug, Selectable, Clone, PartialEq)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(CreditCard))]
#[diesel(table_name = wallet_card_attempt)]
pub struct WalletCardAttempt {
    pub id: i32,
    pub public_id: Uuid,
    pub user_id: i32,
    pub credit_card_id: i32,
    pub expected_reference_id: String,
    pub status: WalletCardAttemptStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Clone)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(CreditCard))]
#[diesel(table_name = wallet_card_attempt)]
pub struct InsertableCardAttempt<'a> {
    pub user_id: i32,
    pub credit_card_id: i32,
    pub expected_reference_id: &'a str,
}

#[derive(Serialize, Deserialize, AsChangeset, Clone, Debug)]
#[diesel(table_name = wallet_card_attempt)]
pub struct UpdateCardAttempt {
    pub status: WalletCardAttemptStatus
}


#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug, Selectable, Clone, PartialEq)]
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

#[derive(Insertable, Debug)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(CreditCard))]
#[diesel(table_name = wallet)]
pub struct InsertableCard<'a> {
    pub user_id: i32,
    pub payment_method_id: &'a str,
    pub credit_card_id: i32,
    pub wallet_card_attempt_id: i32,
}

#[derive(Queryable, Debug)]
pub struct WalletWithExtraInfo {
    pub id: i32,
    pub public_id: Uuid,
    pub created_at: NaiveDateTime,
    pub card_name: String,
    pub issuer_name: String,
    pub card_type: String,
    pub card_image_url: String,
}

impl Wallet {
    #[tracing::instrument]
    pub async fn find_all_for_user(user: &User) -> Result<Vec<Self>, DataError> {
        let mut conn = db::connection().await?;
        //let cards = Wallet::belonging_to(&user).load::<Wallet>(&mut conn).await?;
        let cards = wallet::table.filter(
            wallet::user_id.eq(user.id)
        ).load::<Wallet>(&mut conn).await?;
        Ok(cards) 
    }

    #[tracing::instrument]
    //TODO: This is going to break violently
    pub async fn find_all_for_user_with_card_info(user: &User) -> Result<Vec<WalletWithExtraInfo>, DataError> {
        let mut conn = db::connection().await?;
        let cards = wallet::table
        .inner_join(
            credit_card::table
                .inner_join(credit_card_issuer::table)
                .inner_join(credit_card_type::table)
        )
        .filter(
            wallet::user_id.eq(user.id)
        )
        .select(
            (
                wallet::id,
                wallet::public_id,
                wallet::created_at,
                credit_card::name,
                credit_card_issuer::name,
                credit_card_type::name,
                credit_card::card_image_url
            )
        )
        .load::<WalletWithExtraInfo>(&mut conn).await?;
        Ok(cards)
    }

    // TODO: from consumes
    #[tracing::instrument]
    pub async fn insert_card<'a>(card: &InsertableCard<'a>) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        //let insertable_card = InsertableCard::from(card);
        let inserted_card = diesel::insert_into(wallet::table)
        .values(card)
        .get_result(&mut conn).await?;
        Ok(inserted_card)

    }

    #[cfg(test)]
    #[tracing::instrument]
    pub async fn delete(id: i32) -> Result<usize, DataError> {
        let mut conn = db::connection().await?;

        let res = diesel::delete(
                wallet::table
                    .filter(wallet::id.eq(id))
            )
            .execute(&mut conn).await?;
        Ok(res)
    }

    #[cfg(test)]
    #[tracing::instrument]
    pub async fn delete_self(&self) -> Result<usize, DataError> {
        Wallet::delete(self.id).await
    }
}


impl WalletCardAttempt {
    #[tracing::instrument]
    pub async fn insert<'a>(card_attempt: &InsertableCardAttempt<'a>) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;

        let wallet = diesel::insert_into(wallet_card_attempt::table)
        .values(card_attempt)
        .get_result::<WalletCardAttempt>(&mut conn).await?;
        Ok(wallet)
    }

    #[tracing::instrument]
    pub async fn find_by_reference_id(reference: &str) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;

        let card_attempt = wallet_card_attempt::table
            .filter(wallet_card_attempt::expected_reference_id.eq(reference))
            .first(&mut conn).await?;

        Ok(card_attempt)
    }

    #[tracing::instrument]
    pub async fn update_card(id: i32, card: &UpdateCardAttempt) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;

        let wallet = diesel::update(wallet_card_attempt::table)
            .filter(wallet_card_attempt::id.eq(id))
            .set(card)
            .get_result::<WalletCardAttempt>(&mut conn).await?;
        Ok(wallet)
    }

    #[cfg(test)]
    #[tracing::instrument]
    pub async fn delete(id: i32) -> Result<usize, DataError> {
        let mut conn = db::connection().await?;

        let res = diesel::delete(
            wallet_card_attempt::table
                .filter(wallet_card_attempt::id.eq(id))
        )
            .execute(&mut conn).await?;
        Ok(res)
    }

    #[cfg(test)]
    #[tracing::instrument]
    pub async fn delete_self(&self) -> Result<usize, DataError> {
        WalletCardAttempt::delete(self.id).await
    }
}