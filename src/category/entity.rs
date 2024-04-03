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

#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug, Clone)]
#[diesel(table_name = category)]
pub struct Category {
    pub id: i32,
    pub public_id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug, Clone)]
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
    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn get_by_name(name: &str) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let cat = category::table.filter(
            category::name.eq(name.to_lowercase())
        ).first(&mut conn).await?;
        Ok(cat)
    }
}

impl MccMapping {
    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn get_by_mcc(mcc: &str) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let mapping = mcc_mapping::table.filter(
            mcc_mapping::mcc_code.eq(mcc)
        ).first(&mut conn).await?;
        Ok(mapping)
    }
}


#[cfg(test)]
mod test {
    use actix_web::test;
    use crate::category::constant::Category as CategoryEnum;
    use crate::category::entity::{Category, MccMapping};
    use crate::error::data_error::DataError;

    const DINING_MCC: &str = "5812";

    #[test]
    async fn test_get_mcc_mapping_by_mcc() {
        crate::test_helper::general::init();
        let mapping = MccMapping::get_by_mcc(DINING_MCC).await.expect("finds");
        assert_eq!(mapping.mcc_code, DINING_MCC);
        assert_eq!(mapping.category_id, CategoryEnum::Dining as i32);
    }

    #[test]
    async fn test_get_mcc_mapping_by_mcc_not_found() {
        crate::test_helper::general::init();
        let error = MccMapping::get_by_mcc("this is not a valid mcc").await.expect_err("does not find");
        assert_eq!(DataError::NotFound("test".into()), error);
    }

    #[test]
    async fn test_get_category_by_name_case_insensitive() {
        crate::test_helper::general::init();
        let expected_category_name = "hotels";
        let expected_category_id = 1;

        let mut cat = Category::get_by_name("hotels").await.expect("finds");
        assert_eq!(cat.id, expected_category_id);
        assert_eq!(cat.name, expected_category_name);
        cat = Category::get_by_name("HOTELS").await.expect("finds");
        assert_eq!(cat.id, expected_category_id);
        assert_eq!(cat.name, expected_category_name);
        cat = Category::get_by_name("Hotels").await.expect("finds");
        assert_eq!(cat.id, expected_category_id);
        assert_eq!(cat.name, expected_category_name);
        cat = Category::get_by_name("HoTeLs").await.expect("finds");
        assert_eq!(cat.id, expected_category_id);
        assert_eq!(cat.name, expected_category_name);
    }

    #[test]
    async fn test_get_category_by_name_not_found() {
        crate::test_helper::general::init();
        let error = Category::get_by_name("a hotel").await.expect_err("does not find");
        assert_eq!(DataError::NotFound("test".into()), error);
    }
}