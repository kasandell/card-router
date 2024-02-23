use crate::category::entity::{Category, InsertableCategory, InsertableMccMapping, MccMapping};
use crate::data_error::DataError;

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait CategoryDaoTrait {
    fn create(&self, category: InsertableCategory) -> Result<Category, DataError>;
    fn delete_by_id(&self, id: i32) -> Result<usize, DataError>;
    fn delete(&self, category: &Category) -> Result<usize, DataError>;
}

#[cfg_attr(test, automock)]
pub trait MccMappingDaoTrait {
    fn create(&self, mapping: InsertableMccMapping) -> Result<MccMapping, DataError>;
    fn delete_by_id(&self, id: i32) -> Result<usize, DataError>;
    fn delete(&self, mapping: &MccMapping) -> Result<usize, DataError>;
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

    fn delete_by_id(&self, id: i32) -> Result<usize, DataError> {
        Category::delete(id)
    }

    fn delete(&self, category: &Category) -> Result<usize, DataError> {
        category.delete_self()
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

    fn delete_by_id(&self, id: i32) -> Result<usize, DataError> {
        MccMapping::delete(id)
    }

    fn delete(&self, mapping: &MccMapping) -> Result<usize, DataError> {
        mapping.delete_self()
    }
}
