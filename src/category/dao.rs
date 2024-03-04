use std::sync::Arc;
use crate::category::entity::{Category, InsertableCategory, InsertableMccMapping, MccMapping};
use crate::data_error::DataError;
use async_trait::async_trait;

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait CategoryDaoTrait {
    async fn create(self: Arc<Self>, category: InsertableCategory) -> Result<Category, DataError>;
}

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait MccMappingDaoTrait {
    async fn create(self: Arc<Self>, mapping: InsertableMccMapping) -> Result<MccMapping, DataError>;
    async fn get_by_mcc(self: Arc<Self>, mcc: &str) -> Result<MccMapping, DataError>;
}

pub struct CategoryDao{}

pub struct MccMappingDao{}


// async?
impl CategoryDao {
    pub fn new() -> Self {
        Self{}
    }
}

#[async_trait(?Send)]
impl CategoryDaoTrait for CategoryDao {
    async fn create(self: Arc<Self>, category: InsertableCategory) -> Result<Category, DataError> {
        Category::create(category).await
    }
}

impl MccMappingDao {
    pub fn new() -> Self {
        Self{}
    }
}

#[async_trait(?Send)]
impl MccMappingDaoTrait for MccMappingDao {
    async fn create(self: Arc<Self>, mapping: InsertableMccMapping) -> Result<MccMapping, DataError> {
        MccMapping::create(mapping).await
    }

    async fn get_by_mcc(self: Arc<Self>, mcc: &str) -> Result<MccMapping, DataError> {
        MccMapping::get_by_mcc(mcc).await
    }
}
