use std::fmt::Formatter;
use std::sync::Arc;
use crate::error::data_error::DataError;
use crate::rule::entity::Rule;
use crate::rule::request::CreateRuleRequest;
use async_trait::async_trait;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait RuleDaoTrait {
    async fn create(self: Arc<Self>, new_rule: &CreateRuleRequest) -> Result<Rule, DataError>;
    async fn get_rules_for_card_ids(self: Arc<Self>, ids: &Vec<i32>) -> Result<Vec<Rule>, DataError>;

}

pub struct RuleDao {}


impl RuleDao {
    #[tracing::instrument]
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait(?Send)]
impl RuleDaoTrait for RuleDao {
    #[tracing::instrument(skip(self))]
    async fn create(self: Arc<Self>, new_rule: &CreateRuleRequest) -> Result<Rule, DataError> {
        Rule::create(new_rule).await
    }

    #[tracing::instrument(skip(self))]
    async fn get_rules_for_card_ids(self: Arc<Self>, ids: &Vec<i32>) -> Result<Vec<Rule>, DataError> {
        Rule::get_rules_for_card_ids(ids).await
    }
}