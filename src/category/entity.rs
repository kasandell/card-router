use crate::schema::{
    category,
    mcc_mapping
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug)]
#[diesel(table_name = category)]
pub struct Category {
    pub id: i32,
    pub public_id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug)]
#[diesel(belongs_to(Category))]
#[diesel(table_name = mcc_mapping)]
pub struct MccMapping {
    pub id: i32,
    pub public_id: Uuid,
    pub mcc_code: i32,
    pub category_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}