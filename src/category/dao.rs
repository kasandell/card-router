use crate::category::entity::{Category, InsertableCategory, InsertableMccMapping, MccMapping};
use crate::data_error::DataError;

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait CategoryDaoTrait {
    fn create(&self, category: InsertableCategory) -> Result<Category, DataError>;
}

#[cfg_attr(test, automock)]
pub trait MccMappingDaoTrait {
    fn create(&self, mapping: InsertableMccMapping) -> Result<MccMapping, DataError>;
}

pub struct CategoryDao{}

pub struct MccMappingDao{}


impl CategoryDao {
    pub fn new() -> Self {
        Self{}
    }
}

impl CategoryDaoTrait for CategoryDao {
    fn create(&self, category: InsertableCategory) -> Result<Category, DataError> {
        Category::create(category)
    }
}

impl MccMappingDao {
    pub fn new() -> Self {
        Self{}
    }
}

impl MccMappingDaoTrait for MccMappingDao {
    fn create(&self, mapping: InsertableMccMapping) -> Result<MccMapping, DataError> {
        MccMapping::create(mapping)
    }
}
