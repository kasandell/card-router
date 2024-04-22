use std::fmt::Formatter;
use std::sync::Arc;
use crate::category::entity::{Category, MccMapping};
use crate::error::data_error::DataError;
use async_trait::async_trait;
#[cfg(not(feature = "no-redis"))]
use crate::redis::key::Key;
#[cfg(not(feature = "no-redis"))]
use crate::redis::services::{
    RedisService,
    RedisServiceTrait
};
#[cfg(not(feature = "no-redis"))]
use crate::redis::helper::try_redis_fallback_db;

#[cfg(test)]
use mockall::{automock, predicate::*};

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
    #[cfg(not(feature = "no-redis"))]
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
        #[cfg(not(feature = "no-redis"))]
        {
            Self {
                redis: Arc::new(RedisService::new())
            }
        }
        #[cfg(feature = "no-redis")]
        {
            Self {}
        }

    }
}

#[async_trait(?Send)]
impl MccMappingDaoTrait for MccMappingDao {

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn get_by_mcc(self: Arc<Self>, mcc: &str) -> Result<MccMapping, DataError> {
        #[cfg(not(feature = "no-redis"))]
        {
            Ok(try_redis_fallback_db(
                self.redis.clone(),
                Key::MccMapping(mcc),
                || async { MccMapping::get_by_mcc(mcc).await },
                false
            ).await?)
        }
        #[cfg(feature = "no-redis")]
        {
            Ok(MccMapping::get_by_mcc(mcc).await?)
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