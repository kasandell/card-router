use crate::category::entity::{Category, InsertableCategory, InsertableMccMapping, MccMapping};
use crate::data_error::DataError;
use async_trait::async_trait;

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
#[async_trait]
pub trait CategoryDaoTrait {
    async fn create(&self, category: InsertableCategory) -> Result<Category, DataError>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait MccMappingDaoTrait {
    async fn create(&self, mapping: InsertableMccMapping) -> Result<MccMapping, DataError>;
}

pub struct CategoryDao{}

pub struct MccMappingDao{}


// async?
impl CategoryDao {
    pub fn new() -> Self {
        Self{}
    }
}

#[async_trait]
impl CategoryDaoTrait for CategoryDao {
    async fn create(&self, category: InsertableCategory) -> Result<Category, DataError> {
        Category::create(category).await
    }
}

impl MccMappingDao {
    pub async fn new() -> Self {
        Self{}
    }
}

#[async_trait]
impl MccMappingDaoTrait for MccMappingDao {
    async fn create(&self, mapping: InsertableMccMapping) -> Result<MccMapping, DataError> {
        MccMapping::create(mapping).await
    }
}
