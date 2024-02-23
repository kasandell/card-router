use crate::data_error::DataError;
use crate::rule_engine::entity::Rule;
use crate::rule_engine::request::CreateRuleRequest;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait RuleDaoTrait {
    fn create(&self, new_rule: CreateRuleRequest) -> Result<Rule, DataError>;
    fn get_rules_for_card_ids(&self, ids: &Vec<i32>) -> Result<Vec<Rule>, DataError>;

}

pub struct RuleDao {}

impl RuleDao {
    pub fn new() -> Self {
        Self {}
    }
}

impl RuleDaoTrait for RuleDao {
    fn create(&self, new_rule: CreateRuleRequest) -> Result<Rule, DataError> {
        Rule::create(new_rule)
    }

    fn get_rules_for_card_ids(&self, ids: &Vec<i32>) -> Result<Vec<Rule>, DataError> {
        Rule::get_rules_for_card_ids(ids)
    }
}