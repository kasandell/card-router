use std::ops::DerefMut;
use std::sync::Arc;
use crate::{credit_card_type::model::{
    CreditCardIssuerModel as CreditCardIssuer,
    CreditCardModel as CreditCard,
    CreditCardTypeModel as CreditCardType
}, schema::{
    credit_card,
    credit_card_issuer,
    credit_card_type,
    wallet,
    wallet_card_attempt,
    wallet_status_history
}};
use crate::util::db;
use crate::error::data_error::DataError;
use crate::user::model::UserModel as User;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::result::Error;
use diesel_async::{AsyncConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use parking_lot::Mutex;
use uuid::Uuid;
use uuidv7;
use crate::util::transaction::Transaction;
use crate::wallet::constant::{WalletCardAttemptStatus, WalletStatus};

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
    pub status: WalletStatus,
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

#[derive(Serialize, Deserialize, AsChangeset, Clone, Debug)]
#[diesel(table_name = wallet)]
pub struct UpdateWalletStatus {
    pub status: WalletStatus
}

#[derive(Queryable, Debug)]
pub struct WalletWithExtraInfo {
    pub id: i32,
    pub public_id: Uuid,
    pub status: WalletStatus,
    pub created_at: NaiveDateTime,
    pub card_name: String,
    pub issuer_name: String,
    pub card_type: String,
    pub card_image_url: String,
}

#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug, Selectable, Clone, PartialEq)]
#[diesel(belongs_to(Wallet))]
#[diesel(table_name = wallet_status_history)]
pub struct WalletStatusHistory {
    pub id: i32,
    pub public_id: Uuid,
    pub wallet_id: i32,
    pub prior_status: WalletStatus,
    pub current_status: WalletStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(belongs_to(Wallet))]
#[diesel(table_name = wallet_status_history)]
pub struct InsertableWalletStatusHistory {
    pub wallet_id: i32,
    pub prior_status: WalletStatus,
    pub current_status: WalletStatus,
}

impl Wallet {
    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn find_all_for_user(user: &User) -> Result<Vec<Self>, DataError> {
        let mut conn = db::connection().await?;
        //let cards = Wallet::belonging_to(&user).load::<Wallet>(&mut conn).await?;
        let cards = wallet::table.filter(
            wallet::user_id.eq(user.id)
        ).load::<Wallet>(&mut conn).await?;
        Ok(cards) 
    }


    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn find_by_public_id(public_id: &Uuid) -> Result<Wallet, DataError> {
        let mut conn = db::connection().await?;
        let card = wallet::table.filter(
            wallet::public_id.eq(public_id)
        ).first::<Wallet>(&mut conn).await?;
        Ok(card)

    }

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
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
                wallet::status,
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

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn insert_card<'a>(transaction: Arc<Transaction<'a>>, card: &InsertableCard<'a>) -> Result<Self, DataError> {
        let res = transaction.lock().transaction::<_, diesel::result::Error, _>(|mut _conn| Box::pin(async move {
            let inserted_card = diesel::insert_into(wallet::table)
                .values(card)
                .get_result::<Wallet>(&mut _conn).await?;
            Ok(inserted_card)

        })).await?;
        Ok(res)
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn update_card_status<'a>(transaction: Arc<Transaction<'a>>, id: i32, status_update: &UpdateWalletStatus) -> Result<Self, DataError> {
        let wallet = diesel::update(wallet::table)
            .filter(wallet::id.eq(id))
            .set(status_update)
            .get_result::<Wallet>(&mut transaction.lock()).await?;
        Ok(wallet)
    }

    #[cfg(test)]
    #[cfg_attr(feature="trace-detail", tracing::instrument)]
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
    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn delete_self(&self) -> Result<usize, DataError> {
        Wallet::delete(self.id).await
    }
}


impl WalletCardAttempt {
    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn insert<'a>(card_attempt: &InsertableCardAttempt<'a>) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;

        let wallet = diesel::insert_into(wallet_card_attempt::table)
        .values(card_attempt)
        .get_result::<WalletCardAttempt>(&mut conn).await?;
        Ok(wallet)
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn find_by_reference_id(reference: &str) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;

        let card_attempt = wallet_card_attempt::table
            .filter(wallet_card_attempt::expected_reference_id.eq(reference))
            .first(&mut conn).await?;

        Ok(card_attempt)
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn update_card<'a>(transaction: Arc<Transaction<'a>>, id: i32, card: &UpdateCardAttempt) -> Result<Self, DataError> {

        let wallet = diesel::update(wallet_card_attempt::table)
            .filter(wallet_card_attempt::id.eq(id))
            .set(card)
            .get_result::<WalletCardAttempt>(&mut transaction.lock()).await?;
        Ok(wallet)
    }

    #[cfg(test)]
    #[cfg_attr(feature="trace-detail", tracing::instrument)]
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
    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn delete_self(&self) -> Result<usize, DataError> {
        WalletCardAttempt::delete(self.id).await
    }
}

impl WalletStatusHistory {
    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn insert_status_update<'a>(transaction: Arc<Transaction<'a>>, history: &InsertableWalletStatusHistory) -> Result<Self, DataError> {
            let inserted_history = diesel::insert_into(wallet_status_history::table)
                .values(history)
                .get_result::<WalletStatusHistory>(&mut transaction.lock()).await?;
            Ok(inserted_history)
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn get(id: i32) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let history = wallet_status_history::table.filter(
            wallet_status_history::id.eq(id)
        ).first::<WalletStatusHistory>(&mut conn).await?;
        Ok(history)
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn get_by_wallet_id(wallet_id: i32) -> Result<Vec<Self>, DataError> {
        let mut conn = db::connection().await?;
        let history = wallet_status_history::table.filter(
            wallet_status_history::wallet_id.eq(wallet_id)
        )
            .order_by(wallet_status_history::id.asc())
            .load::<WalletStatusHistory>(&mut conn).await?;
        Ok(history)
    }
}