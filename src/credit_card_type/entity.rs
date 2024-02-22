use crate::schema::{credit_card, credit_card_issuer, credit_card_type, wallet};

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::data_error::DataError;
use crate::user::entity::User;
use crate::util::db;
use crate::wallet::entity::Wallet;


#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Identifiable, Selectable)]
#[diesel(table_name = credit_card)]
#[diesel(belongs_to(CreditCardIssuer))]//, foreign_key="card_issuer"))]
#[diesel(belongs_to(CreditCardType))]//, foreign_key="card_type"))]
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

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Identifiable, Selectable)]
#[diesel(table_name = credit_card_type)]
pub struct CreditCardType {
    pub id: i32,
    pub public_id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Identifiable, Selectable)]
#[diesel(table_name = credit_card_issuer)]
pub struct CreditCardIssuer {
    pub id: i32,
    pub public_id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

impl CreditCard {
    pub fn list_all_card_types() -> Result<Vec<(Self, CreditCardType, CreditCardIssuer)>, DataError> {
        let mut conn = db::connection()?;
        let cards = credit_card::table
            .inner_join(credit_card_type::table)
            .inner_join(credit_card_issuer::table)
            .select((Self::as_select(), CreditCardType::as_select(), CreditCardIssuer::as_select()))
            .load::<(Self, CreditCardType, CreditCardIssuer)>(&mut conn)?;
        info!("Query executed ok");
        Ok(cards)
    }

    pub fn search_all_card_types(
        query: String
    ) -> Result<Vec<(Self, CreditCardType, CreditCardIssuer)>, DataError> {
        let mut conn = db::connection()?;
        let cards = credit_card::table
            .inner_join(credit_card_type::table)
            .inner_join(credit_card_issuer::table)
            .filter(credit_card::name.like(&query).or(
                credit_card_type::name.like(&query).or(
                    credit_card_issuer::name.like(&query)
                )
            ))
            .select((Self::as_select(), CreditCardType::as_select(), CreditCardIssuer::as_select()))
            .load::<(Self, CreditCardType, CreditCardIssuer)>(&mut conn)?;
        info!("Query executed ok");
        Ok(cards)
    }
}

#[cfg(test)]
impl CreditCard {
    pub fn create_test_credit_card(
        id: i32,
        name: String,
        credit_card_type_id: i32,
        credit_card_issuer_id: i32
    ) -> Self {
        CreditCard { 
            id: id, 
            public_id: Uuid::new_v4(), 
            name: name,
            credit_card_type_id: credit_card_type_id,
            credit_card_issuer_id: credit_card_issuer_id,
            card_image_url: "".to_string(),
            created_at: Utc::now().naive_utc(), 
            updated_at: Utc::now().naive_utc()
        }
    }
}

#[cfg(test)]
impl CreditCardIssuer {
    pub fn create_test_credit_card_issuer(id: i32, name: String) -> Self {
        CreditCardIssuer { 
            id: id, 
            public_id: Uuid::new_v4(), 
            name: name,
            created_at: Utc::now().naive_utc(), 
            updated_at: Utc::now().naive_utc()
        }
    }

}

#[cfg(test)]
impl CreditCardType {
    pub fn create_test_credit_card_type(id: i32, name: String) -> Self {
        CreditCardType { 
            id: id, 
            public_id: Uuid::new_v4(), 
            name: name,
            created_at: Utc::now().naive_utc(), 
            updated_at: Utc::now().naive_utc()
        }
    }

}