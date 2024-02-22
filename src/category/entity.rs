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

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = category)]
pub struct InsertableCategory {
    pub name: String
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
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
    pub fn create(category: InsertableCategory) -> Result<Self, DataError>{
        let mut conn = db::connection()?;
        let cat = diesel::insert_into(category::table)
        .values(category)
        .get_result(&mut conn)?;
        Ok(cat)
    }

    #[cfg(test)]
    pub fn delete(id: i32) -> Result<usize, DataError> {
        let mut conn = db::connection()?;

        let res = diesel::delete(
                category::table
                    .filter(category::id.eq(id))
            )
            .execute(&mut conn)?;
        Ok(res)
    }

    #[cfg(test)]
    pub fn delete_self(&self) -> Result<usize, DataError> {
        Category::delete(self.id)
    }
}

impl MccMapping {
    pub fn create(mapping: InsertableMccMapping) -> Result<Self, DataError> {
        let mut conn = db::connection()?;
        let map = diesel::insert_into(mcc_mapping::table)
        .values(mapping)
        .get_result(&mut conn)?;
        Ok(map)
    }

    #[cfg(test)]
    pub fn delete(id: i32) -> Result<usize, DataError> {
        let mut conn = db::connection()?;

        let res = diesel::delete(
                mcc_mapping::table
                    .filter(mcc_mapping::id.eq(id))
            )
            .execute(&mut conn)?;
        Ok(res)
    }

    #[cfg(test)]
    pub fn delete_self(&self) -> Result<usize, DataError> {
        MccMapping::delete(self.id)
    }
}