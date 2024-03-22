use crate::{schema::{
    wallet,
    wallet_card_attempt,
    credit_card,
    credit_card_issuer,
    credit_card_type
}, credit_card_type::model::{
    CreditCardTypeModel as CreditCardType,
    CreditCardModel as CreditCard,
    CreditCardIssuerModel as CreditCardIssuer
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
pub struct InsertableCard {
    pub public_id: Uuid,
    pub user_id: i32,
    pub payment_method_id: String,
    pub credit_card_id: i32,
    pub wallet_card_attempt_id: i32,
}

#[derive(Serialize, Deserialize, AsChangeset, Debug)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(CreditCard))]
#[diesel(table_name = wallet)]
pub struct NewCard<'a> {
    pub user_id: i32,
    pub payment_method_id: &'a str,
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
    pub async fn find_all_for_user_with_card_info(user: &User) -> Result<Vec<WalletDetail>, DataError> {
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
        .select((Wallet::as_select(), CreditCard::as_select(), CreditCardType::as_select(), CreditCardIssuer::as_select()))
        .load::<WalletDetail>(&mut conn).await?;
        Ok(cards)
    }

    // TODO: from consumes
    #[tracing::instrument]
    pub async fn insert_card<'a>(card: &NewCard<'a>) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let insertable_card = InsertableCard::from(card);
        let inserted_card = diesel::insert_into(wallet::table)
        .values(insertable_card)
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

impl From<&NewCard<'_>> for InsertableCard {
    fn from(card: &NewCard) -> Self {
        InsertableCard {
            public_id: Uuid::new_v4(),
            user_id: card.user_id,
            payment_method_id: card.payment_method_id.to_string(),
            credit_card_id: card.credit_card_id,
            wallet_card_attempt_id: card.wallet_card_attempt_id
        }
    }
}

#[cfg(test)]
impl Wallet {
    #[tracing::instrument]
    pub async fn create_test_wallet(
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

    #[tracing::instrument]
    pub async fn create_test_wallet_in_db(
        user_id: i32,
        credit_card_id: i32
    ) -> Result<(Self, WalletCardAttempt), DataError> {
        let ca = WalletCardAttempt::insert(
            &InsertableCardAttempt {
                user_id: user_id,
                credit_card_id: credit_card_id,
                expected_reference_id: "test",
            }
        ).await?;
        let wallet = Wallet::insert_card(
            &NewCard {
                user_id: user_id,
                payment_method_id: "test",
                credit_card_id: credit_card_id,
                wallet_card_attempt_id: ca.id,

            }
        ).await?;
        Ok((wallet, ca))
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