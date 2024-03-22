use async_trait::async_trait;
use std::sync::Arc;
use super::entity::{Category, MccMapping};
use super::error::CategoryError;
#[cfg(test)]
use mockall::automock;
use crate::category::dao::{CategoryDao, CategoryDaoTrait, MccMappingDao, MccMappingDaoTrait};
use crate::category::model::{CategoryModel, MccMappingModel};


// TODO: all future services should return only objects exposed in request / response
#[async_trait(?Send)]
#[cfg_attr(test, automock)]
pub trait CategoryServiceTrait {
    async fn get_category_by_name(self: Arc<Self>, name: &str) -> Result<Category, CategoryError>;
    async fn get_mcc_mapping_by_mcc(self: Arc<Self>, mcc: &str) -> Result<MccMapping, CategoryError>;
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
        Ok(self.category_dao.clone().get_by_name(name).await?.into())
    }

    #[tracing::instrument(skip(self))]
    async fn get_mcc_mapping_by_mcc(self: Arc<Self>, mcc: &str) -> Result<MccMappingModel, CategoryError> {
        Ok(self.mcc_dao.clone().get_by_mcc(mcc).await?.into())
    }

}