use crate::schema::passthrough_card;
use chrono::{NaiveDateTime, NaiveDate, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use lithic_client::models::{Card, FundingAccount};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::data_error::DataError;
use crate::error_type::ErrorType;
use crate::user::entity::User;
use crate::util::date::expiration_date_from_str_parts;
use crate::util::db;
use super::constant::{
    PassthroughCardStatus,
    PassthroughCardType
};

#[derive(Clone, Debug)]
pub struct LithicCard {
    pub token: String,
    pub last_four: String,
    pub exp_month: String,
    pub exp_year: String
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Identifiable, Clone)]
#[diesel(table_name = passthrough_card)]
#[diesel(belongs_to(PassthroughCardType))]
#[diesel(belongs_to(PassThroughCardStatus))]
pub struct PassthroughCard {
    pub id: i32,
    pub public_id: Uuid,
    pub passthrough_card_status: String,
    pub is_active: Option<bool>,
    pub user_id: i32,
    pub token: String,
    pub expiration: NaiveDate,
    pub last_four: String,
    pub passthrough_card_type: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = passthrough_card)]
#[diesel(belongs_to(PassthroughCardType))]
#[diesel(belongs_to(PassThroughCardStatus))]
pub struct InsertablePassthroughCard {
    pub passthrough_card_status: String,
    pub public_id: Uuid,
    pub user_id: i32,
    pub token: String,
    pub expiration: NaiveDate,
    pub last_four: String,
    pub passthrough_card_type: String,
    pub is_active: bool
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = passthrough_card)]
#[diesel(belongs_to(PassThroughCardStatus))]
#[diesel(treat_none_as_null = true)]
pub struct PassthroughCardStatusUpdate<'a> {
    pub id: i32,
    pub passthrough_card_status: &'a str,
    pub is_active: Option<bool>
}


impl PassthroughCard {
    pub async fn create(card: LithicCard, user: &User) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;

        let mut card = InsertablePassthroughCard::from(card);
        card.user_id = user.id;
        //TODO: populate with user_id
        let card = diesel::insert_into(passthrough_card::table)
            .values(card)
            .get_result(&mut conn).await?;
        Ok(card)
    }

    pub async fn create_from_api_card(card: &Card, user: &User) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;

        let mut card = InsertablePassthroughCard::convert_from(&card)?;
        card.user_id = user.id;
        //TODO: populate with user_id
        let card = diesel::insert_into(passthrough_card::table)
            .values(card)
            .get_result(&mut conn).await?;
        Ok(card)
    }

    pub async fn update_status(id: i32, status: PassthroughCardStatus) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let update = PassthroughCardStatusUpdate {
            id: id,
            passthrough_card_status: &String::from(&status),
            is_active: status.is_active_for_status()
        };
        let update = diesel::update(passthrough_card::table)
        .filter(passthrough_card::id.eq(id))
        .set(update)
        .get_result(&mut conn).await?;
        Ok(update)
    }

    pub async fn get(id: i32) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;

        let card = passthrough_card::table
            .filter(passthrough_card::id.eq(id))
            .first(&mut conn).await?;
        Ok(card)
    }

    pub async fn get_by_token(token: &str) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;

        let card = passthrough_card::table
            .filter(passthrough_card::token.eq(token))
            .first(&mut conn).await?;
        Ok(card)
    }

    pub async fn find_cards_for_user(user_id: i32) -> Result<Vec<Self>, DataError> {
        let mut conn = db::connection().await?;

        let cards = passthrough_card::table
            .filter(passthrough_card::user_id.eq(user_id))
            .load::<PassthroughCard>(&mut conn).await?;
        Ok(cards)
    }

    pub async fn find_card_for_user_in_status(
        user_id: i32,
        status: PassthroughCardStatus
    ) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;

        let card = passthrough_card::table
            .filter(
                passthrough_card::user_id.eq(user_id)
                    .and(
                        passthrough_card::passthrough_card_status.eq(String::from(&status))
                    )
            )
            .order(passthrough_card::id.desc())
            .first(&mut conn).await?;
        Ok(card)
    }

    #[cfg(test)]
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
    pub async fn delete_self(&self) -> Result<usize, DataError> {
        PassthroughCard::delete(self.id).await
    }

}

impl From<LithicCard> for InsertablePassthroughCard {
    fn from(card: LithicCard) -> Self {
        InsertablePassthroughCard {
            passthrough_card_status: String::from(&PassthroughCardStatus::OPEN),
            public_id: Uuid::new_v4(),
            user_id: 0,
            token: card.token,
            expiration: expiration_date_from_str_parts(&card.exp_year, &card.exp_month).expect("Expiration needs to be valid"),
            last_four: card.last_four,
            passthrough_card_type: String::from(&PassthroughCardType::VIRTUAL),
            is_active: true
        }
    }
}

// async.await?
impl InsertablePassthroughCard {
    pub fn convert_from(card: &Card) -> Result<Self, DataError> {
        let exp_year = card.exp_year.clone().ok_or(
            DataError::new(ErrorType::InternalServerError, "Cannot find expiration year")
        )?;
        let exp_month = card.exp_month.clone().ok_or(
            DataError::new(ErrorType::InternalServerError, "Cannot find expiration month")
        )?;
        let expiration = expiration_date_from_str_parts(&exp_year, &exp_month)?;
        Ok(InsertablePassthroughCard {
            passthrough_card_status: String::from(&PassthroughCardStatus::OPEN),
            public_id: Uuid::new_v4(),
            user_id: 0,
            token: card.token.to_string(),
            expiration: expiration,
            last_four: card.last_four.clone(),
            passthrough_card_type: String::from(&PassthroughCardType::VIRTUAL),
            is_active: true
        })
    }
}

#[cfg(test)]
pub fn create_test_lithic_card(
    exp_month: String,
    exp_year: String,
    last_four: String,
    token: Uuid
) -> Card {
    Card {
        created: "".to_string(),
        cvv: None,
        funding: Box::new(FundingAccount {
            account_name: None,
            created: "".to_string(),
            last_four: "".to_string(),
            nickname: None,
            state: Default::default(),
            token: Default::default(),
            r#type: Default::default(),
        }),
        exp_month: Some(exp_month),
        exp_year: Some(exp_year),
        hostname: None,
        last_four: last_four,
        memo: None,
        pan: None,
        spend_limit: 0,
        spend_limit_duration: Default::default(),
        state: Default::default(),
        auth_rule_tokens: None,
        token: Default::default(),
        r#type: Default::default(),
        digital_card_art_token: None,
    }
}
