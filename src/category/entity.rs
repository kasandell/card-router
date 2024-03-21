use crate::{
    schema::{
        category,
        mcc_mapping
    },
    error::data_error::DataError,
    util::db
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use diesel_async::RunQueryDsl;
use crate::error::data_error::DataError;

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
    pub mcc_code: String,
    pub category_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Category {
    #[tracing::instrument]
    pub async fn get_by_name(name: &str) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let cat = category::table.filter(
            category::name.eq(name)
        ).first(&mut conn).await?;
        Ok(cat)
    }
}

impl MccMapping {
    #[tracing::instrument]
    pub async fn get_by_mcc(mcc: &str) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let mapping = mcc_mapping::table.filter(
            mcc_mapping::mcc_code.eq(mcc)
        ).first(&mut conn).await?;
        Ok(mapping)
    }
}