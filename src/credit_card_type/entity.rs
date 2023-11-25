use crate::schema::{
    credit_card,
    credit_card_issuer,
    credit_card_type
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Identifiable)]
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

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Identifiable)]
#[diesel(table_name = credit_card_type)]
pub struct CreditCardType {
    pub id: i32,
    pub public_id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Identifiable)]
#[diesel(table_name = credit_card_issuer)]
pub struct CreditCardIssuer {
    pub id: i32,
    pub public_id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}