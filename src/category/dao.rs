use std::fmt::Formatter;
use std::sync::Arc;
use crate::category::entity::{Category, MccMapping};
use crate::error::data_error::DataError;
use async_trait::async_trait;

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

pub struct MccMappingDao{}

// async?
impl CategoryDao {

    #[tracing::instrument]
    pub fn new() -> Self {
        Self{}
    }
}

#[async_trait(?Send)]
impl CategoryDaoTrait for CategoryDao {
    #[tracing::instrument(skip(self))]
    async fn get_by_name(self: Arc<Self>, name: &str) -> Result<Category, DataError> {
        Category::get_by_name(name)?
    }
}

impl MccMappingDao {
    #[tracing::instrument]
    pub fn new() -> Self {
        Self{}
    }
}

#[async_trait(?Send)]
impl MccMappingDaoTrait for MccMappingDao {

    #[tracing::instrument(skip(self))]
    async fn get_by_mcc(self: Arc<Self>, mcc: &str) -> Result<MccMapping, DataError> {
        MccMapping::get_by_mcc(mcc).await
    }
}
