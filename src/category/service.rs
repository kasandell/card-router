use async_trait::async_trait;
use std::sync::Arc;
use super::entity::{Category, MccMapping};
use super::error::CategoryError;
use crate::category::dao::{CategoryDao, CategoryDaoTrait, MccMappingDao, MccMappingDaoTrait};
use crate::category::model::{CategoryModel, MccMappingModel};


// TODO: all future services should return only objects exposed in request / response
#[async_trait(?Send)]
pub trait CategoryServiceTrait {
    async fn get_category_by_name(self: Arc<Self>, name: &str) -> Result<CategoryModel, CategoryError>;
    async fn get_mcc_mapping_by_mcc(self: Arc<Self>, mcc: &str) -> Result<MccMappingModel, CategoryError>;
}


pub struct CategoryService {
    category_dao: Arc<dyn CategoryDaoTrait>,
    mcc_dao: Arc<dyn MccMappingDaoTrait>
}

impl CategoryService {
    pub fn new() -> Self {
        Self {
            category_dao: Arc::new(CategoryDao::new()),
            mcc_dao: Arc::new(MccMappingDao::new()),
        }
    }

    pub(super) fn new_with_services(
        category_dao: Arc<dyn CategoryDaoTrait>,
        mcc_dao: Arc<dyn MccMappingDaoTrait>
    ) -> Self {
        Self {
            category_dao: category_dao.clone(),
            mcc_dao: mcc_dao.clone()
        }
    }
}

#[async_trait(?Send)]
impl CategoryServiceTrait for CategoryService {
    #[tracing::instrument(skip(self))]
    async fn get_category_by_name(self: Arc<Self>, name: &str) -> Result<CategoryModel, CategoryError> {
        tracing::info!("Getting category by name={}", name);
        Ok(self.category_dao.clone().get_by_name(name).await?.into())
    }

    #[tracing::instrument(skip(self))]
    async fn get_mcc_mapping_by_mcc(self: Arc<Self>, mcc: &str) -> Result<MccMappingModel, CategoryError> {
        tracing::info!("Getting mcc mapping by mcc={}", mcc);
        Ok(self.mcc_dao.clone().get_by_mcc(mcc).await?.into())
    }

}



#[cfg(test)]
mod test {
    use std::sync::Arc;
    use actix_web::test;
    use crate::category::constant::Category as CategoryEnum;
    use crate::category::error::CategoryError;
    use crate::category::service::{CategoryService, CategoryServiceTrait};

    const DINING_MCC: &str = "5812";
    const DINING_CATEGORY_NAME: &str = "dining";

    #[test]
    async fn test_get_category_by_name_ok() {
        let svc = Arc::new(CategoryService::new());
        let res = svc.clone().get_category_by_name(DINING_CATEGORY_NAME).await.expect("Ok");
        assert_eq!(res.name, DINING_CATEGORY_NAME);
        assert_eq!(res.id, CategoryEnum::Dining as i32);
    }

    #[test]
    async fn test_get_category_by_name_not_found() {
        let svc = Arc::new(CategoryService::new());
        let error = svc.clone().get_category_by_name("restaurants are not category named").await.expect_err("Ok");
        assert_eq!(CategoryError::Unexpected("test".into()), error);
    }

    #[test]
    async fn test_get_mcc_mapping_by_mcc_ok() {
        let svc = Arc::new(CategoryService::new());
        let res = svc.clone().get_mcc_mapping_by_mcc(DINING_MCC).await.expect("Ok");
        assert_eq!(res.mcc_code, DINING_MCC);
        assert_eq!(res.category_id, CategoryEnum::Dining as i32);
    }

    #[test]
    async fn test_get_mcc_mapping_by_mcc_not_found() {
        let svc = Arc::new(CategoryService::new());
        let error = svc.clone().get_mcc_mapping_by_mcc("this is not an mcc").await.expect_err("Ok");
        assert_eq!(CategoryError::Unexpected("test".into()), error);

    }

}
