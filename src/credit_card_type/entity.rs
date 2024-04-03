use crate::schema::{credit_card, credit_card_issuer, credit_card_type, wallet};

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::error::data_error::DataError;
use crate::util::db;
use diesel_async::RunQueryDsl;


#[derive(Queryable, Debug, Identifiable, Selectable, Clone)]
#[diesel(table_name = credit_card)]
#[diesel(belongs_to(CreditCardIssuer))]
#[diesel(belongs_to(CreditCardType))]
pub struct CreditCard {
    pub id: i32,
    pub public_id: Uuid,
    pub name: String,
    pub credit_card_type_id: i32,
    pub credit_card_issuer_id: i32,
    pub card_image_url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Queryable, Debug, Identifiable, Selectable, Clone)]
#[diesel(table_name = credit_card_type)]
pub struct CreditCardType {
    pub id: i32,
    pub public_id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Queryable, Debug, Identifiable, Selectable, Clone)]
#[diesel(table_name = credit_card_issuer)]
pub struct CreditCardIssuer {
    pub id: i32,
    pub public_id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

impl CreditCard {
    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn list_all_card_types() -> Result<Vec<(Self, CreditCardType, CreditCardIssuer)>, DataError> {
        let mut conn = db::connection().await?;
        let cards = credit_card::table
            .inner_join(credit_card_type::table)
            .inner_join(credit_card_issuer::table)
            .select((Self::as_select(), CreditCardType::as_select(), CreditCardIssuer::as_select()))
            .load::<(Self, CreditCardType, CreditCardIssuer)>(&mut conn).await?;
        tracing::info!("Query executed ok");
        Ok(cards)
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn find_by_public_id(
        public_id: &Uuid
    ) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;

        let card = credit_card::table
            .filter(credit_card::public_id.eq(public_id))
            .first(&mut conn).await?;

        Ok(card)
    }
}

#[cfg(test)]
mod test {
    use crate::credit_card_type::entity::CreditCard;
    use actix_web::test;
    use uuid::Uuid;
    use crate::error::data_error::DataError;

    #[test]
    pub async fn test_list() {
        crate::test_helper::general::init();
        let cards = CreditCard::list_all_card_types().await.expect("Ok");
        assert_eq!(3, cards.len())
    }

    #[test]
    async fn test_find_by_pub_id_finds() {
        crate::test_helper::general::init();
        // This is a pub id pulled from db. if remigrate db, need ot grab this
        let cards = CreditCard::list_all_card_types().await.expect("OK");
        let id = cards[0].0.public_id;
        let card = CreditCard::find_by_public_id(&id).await.expect("ok");
        assert_eq!(card.id, cards[0].0.id);
        assert_eq!(card.public_id, id);
        assert_eq!(card.name, cards[0].0.name);
    }

    #[test]
    async fn test_find_by_pub_id_does_not_find() {
        crate::test_helper::general::init();
        let id = Uuid::new_v4();
        let error = CreditCard::find_by_public_id(&id).await.expect_err("ok");
        assert_eq!(DataError::NotFound("test".into()), error);
    }
}