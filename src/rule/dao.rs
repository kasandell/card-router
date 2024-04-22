use std::sync::Arc;
use crate::error::data_error::DataError;
use crate::rule::entity::Rule;
use crate::rule::request::CreateRuleRequest;
use async_trait::async_trait;
#[cfg(not(feature = "no-redis"))]
use crate::redis::helper::try_redis_fallback_db;
#[cfg(not(feature = "no-redis"))]
use crate::redis::key::Key;
#[cfg(not(feature = "no-redis"))]
use crate::redis::services::{RedisService, RedisServiceTrait};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait RuleDaoTrait {
    async fn create(self: Arc<Self>, new_rule: &CreateRuleRequest) -> Result<Rule, DataError>;
    async fn get_rules_for_card_ids(self: Arc<Self>, ids: &Vec<i32>) -> Result<Vec<Rule>, DataError>;

}

pub struct RuleDao {
    #[cfg(not(feature = "no-redis"))]
    redis: Arc<RedisService>
}


impl RuleDao {
    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub fn new() -> Self {
        #[cfg(feature = "no-redis")]
        {
            Self {}
        }
        #[cfg(not(feature = "no-redis"))]
        {
            Self {
                redis: Arc::new(RedisService::new())
            }
        }
    }
}

#[async_trait(?Send)]
impl RuleDaoTrait for RuleDao {
    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn create(self: Arc<Self>, new_rule: &CreateRuleRequest) -> Result<Rule, DataError> {
        Rule::create(new_rule).await
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn get_rules_for_card_ids(self: Arc<Self>, ids: &Vec<i32>) -> Result<Vec<Rule>, DataError> {
        #[cfg(not(feature = "no-redis"))] {
            Ok(try_redis_fallback_db(
                self.redis.clone(),
                Key::RulesForCards(ids),
                || async {Rule::get_rules_for_card_ids(ids).await},
                false
            ).await?)
        }
        #[cfg(feature = "no-redis")] {
            Rule::get_rules_for_card_ids(ids).await
        }
    }
}