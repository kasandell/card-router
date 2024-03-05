use crate::{
    schema::{
        category,
        mcc_mapping
    },
    data_error::DataError,
    util::db
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use diesel_async::RunQueryDsl;

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Clone)]
#[diesel(table_name = category)]
pub struct InsertableCategory {
    pub name: String
}

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable, Clone)]
#[diesel(belongs_to(Category))]
#[diesel(table_name = mcc_mapping)]
pub struct InsertableMccMapping {
    pub mcc_code: String,
    pub category_id: i32
}



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
    pub async fn create(category: InsertableCategory) -> Result<Self, DataError>{
        let mut conn = db::connection().await?;
        let cat = diesel::insert_into(category::table)
        .values(category)
        .get_result(&mut conn).await?;
        Ok(cat)
    }

    #[cfg(test)]
    pub async fn delete(id: i32) -> Result<usize, DataError> {
        let mut conn = db::connection().await?;

        let res = diesel::delete(
                category::table
                    .filter(category::id.eq(id))
            )
            .execute(&mut conn).await?;
        Ok(res)
    }

    #[cfg(test)]
    pub async fn delete_self(&self) -> Result<usize, DataError> {
        Category::delete(self.id).await
    }
}

impl MccMapping {
    pub async fn create(mapping: InsertableMccMapping) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let map = diesel::insert_into(mcc_mapping::table)
        .values(mapping)
        .get_result(&mut conn).await?;
        Ok(map)
    }

    pub async fn get_by_mcc(mcc: &str) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let mapping = mcc_mapping::table.filter(
            mcc_mapping::mcc_code.eq(mcc)
        ).first(&mut conn).await?;
        Ok(mapping)
    }

    #[cfg(test)]
    pub async fn delete(id: i32) -> Result<usize, DataError> {
        let mut conn = db::connection().await?;

        let res = diesel::delete(
                mcc_mapping::table
                    .filter(mcc_mapping::id.eq(id))
            )
            .execute(&mut conn).await?;
        Ok(res)
    }

    #[cfg(test)]
    pub async fn delete_self(&self) -> Result<usize, DataError> {
        MccMapping::delete(self.id).await
    }
}