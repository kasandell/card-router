use std::sync::Arc;
use crate::data_error::DataError;
use crate::rule_engine::entity::Rule;
use crate::rule_engine::request::CreateRuleRequest;
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
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait(?Send)]
impl RuleDaoTrait for RuleDao {
    async fn create(self: Arc<Self>, new_rule: &CreateRuleRequest) -> Result<Rule, DataError> {
        Rule::create(new_rule).await
    }

    async fn get_rules_for_card_ids(self: Arc<Self>, ids: &Vec<i32>) -> Result<Vec<Rule>, DataError> {
        Rule::get_rules_for_card_ids(ids).await
    }
}