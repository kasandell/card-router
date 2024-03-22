use crate::schema::passthrough_card;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::error::data_error::DataError;
use crate::util::db;
use super::constant::{
    PassthroughCardStatus,
    PassthroughCardType
};

#[derive(Serialize, Deserialize, Queryable, Insertable, Identifiable, Clone, Debug)]
#[diesel(table_name = passthrough_card)]
#[diesel(belongs_to(PassthroughCardType))]
#[diesel(belongs_to(PassThroughCardStatus))]
pub struct PassthroughCard {
    pub id: i32,
    pub public_id: Uuid,
    pub passthrough_card_status: PassthroughCardStatus,
    pub is_active: Option<bool>,
    pub user_id: i32,
    pub token: String,
    pub expiration: NaiveDate,
    pub last_four: String,
    pub passthrough_card_type: PassthroughCardType,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = passthrough_card)]
#[diesel(belongs_to(PassthroughCardType))]
#[diesel(belongs_to(PassThroughCardStatus))]
pub struct InsertablePassthroughCard {
    pub passthrough_card_status: PassthroughCardStatus,
    pub public_id: Uuid,
    pub user_id: i32,
    pub token: String,
    pub expiration: NaiveDate,
    pub last_four: String,
    pub passthrough_card_type: PassthroughCardType,
    pub is_active: bool
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = passthrough_card)]
#[diesel(belongs_to(PassThroughCardStatus))]
#[diesel(treat_none_as_null = true)]
pub struct PassthroughCardStatusUpdate {
    pub id: i32,
    pub passthrough_card_status: PassthroughCardStatus,
    pub is_active: Option<bool>
}


impl PassthroughCard {
    #[tracing::instrument]
    pub async fn create(card: InsertablePassthroughCard) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;

        let card = diesel::insert_into(passthrough_card::table)
            .values(card)
            .get_result(&mut conn).await?;
        Ok(card)
    }

    #[tracing::instrument]
    pub async fn update_status(id: i32, status: PassthroughCardStatus) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let update = PassthroughCardStatusUpdate {
            id: id,
            passthrough_card_status: status,
            is_active: status.is_active_for_status()
        };
        let update = diesel::update(passthrough_card::table)
        .filter(passthrough_card::id.eq(id))
        .set(update)
        .get_result(&mut conn).await?;
        Ok(update)
    }

    #[tracing::instrument]
    pub async fn get(id: i32) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;

        let card = passthrough_card::table
            .filter(passthrough_card::id.eq(id))
            .first(&mut conn).await?;
        Ok(card)
    }

    #[tracing::instrument]
    pub async fn get_by_token(token: &str) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;

        let card = passthrough_card::table
            .filter(passthrough_card::token.eq(token))
            .first(&mut conn).await?;
        Ok(card)
    }

    #[tracing::instrument]
    pub async fn find_cards_for_user(user_id: i32) -> Result<Vec<Self>, DataError> {
        let mut conn = db::connection().await?;

        let cards = passthrough_card::table
            .filter(passthrough_card::user_id.eq(user_id))
            .load::<PassthroughCard>(&mut conn).await?;
        Ok(cards)
    }

    #[tracing::instrument]
    pub async fn find_card_for_user_in_status(
        user_id: i32,
        status: PassthroughCardStatus
    ) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;

        let card = passthrough_card::table
            .filter(
                passthrough_card::user_id.eq(user_id)
                    .and(
                        passthrough_card::passthrough_card_status.eq(status)
                    )
            )
            .order(passthrough_card::id.desc())
            .first(&mut conn).await?;
        Ok(card)
    }

    #[cfg(test)]
    #[tracing::instrument]
    pub async fn delete(id: i32) -> Result<usize, DataError> {
        let mut conn = db::connection().await?;

        let res = diesel::delete(
            passthrough_card::table
                .filter(passthrough_card::id.eq(id))
        )
            .execute(&mut conn).await?;
        Ok(res)
    }

    #[cfg(test)]
    #[tracing::instrument]
    pub async fn delete_self(&self) -> Result<usize, DataError> {
        PassthroughCard::delete(self.id).await
    }

}

