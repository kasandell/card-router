use std::fmt::Formatter;
use std::sync::Arc;
use crate::category::entity::{Category, MccMapping};
use crate::error::data_error::DataError;
use async_trait::async_trait;

#[cfg(test)]
use mockall::{automock, predicate::*};
use crate::redis::key::Key;
use crate::redis::services::{
    RedisService,
    RedisServiceTrait
};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait CategoryDaoTrait {
    async fn get_by_name(self: Arc<Self>, name: &str) -> Result<Category, DataError>;
}

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait MccMappingDaoTrait {
    async fn get_by_mcc(self: Arc<Self>, mcc: &str) -> Result<MccMapping, DataError>;
}

pub struct CategoryDao{}

pub struct MccMappingDao{
    // TODO: can't dyn this due to type params. might not be an issue
    redis: Arc<RedisService>
}

// async?
impl CategoryDao {

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub fn new() -> Self {
        Self{}
    }
}

#[async_trait(?Send)]
impl CategoryDaoTrait for CategoryDao {
    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn get_by_name(self: Arc<Self>, name: &str) -> Result<Category, DataError> {
        Category::get_by_name(&name.to_lowercase()).await
    }
}

impl MccMappingDao {
    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub fn new() -> Self {
        Self{
            redis: Arc::new(RedisService::new())
        }
    }
}

#[async_trait(?Send)]
impl MccMappingDaoTrait for MccMappingDao {

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn get_by_mcc(self: Arc<Self>, mcc: &str) -> Result<MccMapping, DataError> {
        let redis_response = self.redis.clone().get::<_, MccMapping>(&Key::MccMapping(mcc)).await;
        match redis_response {
            Ok(val) => {
                tracing::info!("Returning from redis");
                Ok(val)
            },
            Err(_) => {
                let mcc_mapping = MccMapping::get_by_mcc(mcc).await?;
                let redis_save = self.redis.clone().set::<_, MccMapping>(&Key::MccMapping(mcc), &mcc_mapping).await;
                match redis_save {
                    Ok(_) => {
                        tracing::info!("Saved in redis");
                    },
                    Err(e) => {
                        tracing::warn!("Error saving in redis {:?}", &e);
                    }
                }
                Ok(mcc_mapping)
            }
        }
    }
}



#[cfg(test)]
mod test {
    use std::sync::Arc;
    use actix_web::test;
    use crate::category::constant::Category as CategoryEnum;
    use crate::category::dao::{CategoryDao, CategoryDaoTrait, MccMappingDao, MccMappingDaoTrait};
    use crate::category::entity::{Category, MccMapping};
    use crate::error::data_error::DataError;

    const DINING_MCC: &str = "5812";

    #[test]
    async fn test_get_mcc_mapping_by_mcc() {
        crate::test_helper::general::init();
        let dao = Arc::new(MccMappingDao::new());
        let mapping = dao.clone().get_by_mcc(DINING_MCC).await.expect("finds");
        assert_eq!(mapping.mcc_code, DINING_MCC);
        assert_eq!(mapping.category_id, CategoryEnum::Dining as i32);
    }

    #[test]
    async fn test_get_mcc_mapping_by_mcc_not_found() {
        crate::test_helper::general::init();
        let dao = Arc::new(MccMappingDao::new());
        let error = dao.clone().get_by_mcc("this is not a valid mcc").await.expect_err("does not find");
        assert_eq!(DataError::NotFound("test".into()), error);
    }

    #[test]
    async fn test_get_category_by_name_case_insensitive() {
        crate::test_helper::general::init();
        let dao = Arc::new(CategoryDao::new());
        let expected_category_name = "hotels";
        let expected_category_id = 1;

        let mut cat = dao.clone().get_by_name("hotels").await.expect("finds");
        assert_eq!(cat.id, expected_category_id);
        assert_eq!(cat.name, expected_category_name);
        cat = dao.clone().get_by_name("HOTELS").await.expect("finds");
        assert_eq!(cat.id, expected_category_id);
        assert_eq!(cat.name, expected_category_name);
        cat = dao.clone().get_by_name("Hotels").await.expect("finds");
        assert_eq!(cat.id, expected_category_id);
        assert_eq!(cat.name, expected_category_name);
        cat = dao.clone().get_by_name("HoTeLs").await.expect("finds");
        assert_eq!(cat.id, expected_category_id);
        assert_eq!(cat.name, expected_category_name);
    }

    #[test]
    async fn test_get_category_by_name_not_found() {
        crate::test_helper::general::init();
        let dao = Arc::new(CategoryDao::new());
        let error = dao.clone().get_by_name("a hotel").await.expect_err("does not find");
        assert_eq!(DataError::NotFound("test".into()), error);
    }
}