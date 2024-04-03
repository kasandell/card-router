use std::collections::{hash_map::Entry, HashMap};
use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use crate::asa::request::AsaRequest;
use crate::category::service::{CategoryServiceTrait, CategoryService};
use crate::category::model::MccMappingModel as MccMapping;
use crate::rule::dao::{RuleDao, RuleDaoTrait};
use crate::rule::error::RuleError;
use crate::user::model::UserModel as User;
use crate::util::date::adjust_recurring_to_date;
use crate::wallet::model::{WalletModel as Wallet, WalletModel};
use crate::wallet::service::{WalletService, WalletServiceTrait};
use super::entity::Rule;


#[mockall::automock]
#[async_trait(?Send)]
pub trait RuleServiceTrait {
    async fn order_user_cards_for_request(self: Arc<Self>, request: &AsaRequest, user: &User) -> Result<Vec<Wallet>, RuleError>;
}



pub struct RuleService {
    category_service: Arc<dyn CategoryServiceTrait>,
    rule_dao: Arc<dyn RuleDaoTrait>,
    wallet_service: Arc<dyn WalletServiceTrait>,
}


#[async_trait(?Send)]
impl RuleServiceTrait for RuleService {

    #[tracing::instrument(skip_all)]
    async fn order_user_cards_for_request(self: Arc<Self>, request: &AsaRequest, user: &User) -> Result<Vec<Wallet>, RuleError> {
        /*
        Given an asa request, and a user, attempt charging against a user's wallet until we get a successful attempt
         */
        //wallet, credit_card, credit_card_type, credit_card_issuer
        tracing::info!("Ordering cards in request for user_id={}", &user.id);
        let amount = request.amount.ok_or_else(|| {
            tracing::error!("No amount supplied in the charge request");
            RuleError::NoAmount("No amount supplied".into())
        })?;
        // TODO: move this to service level call
        tracing::info!("Finding all cards for user");
        let cards = self.wallet_service.clone().find_all_for_user(user)
            .await.map_err(|e| {
            tracing::error!("Error retrieving cards for user_id={} error={:?}", &user.id, &e);
            RuleError::Unexpected(e.into())
        })?;
        let card_type_ids = cards.iter().map(|card_with_info| card_with_info.credit_card_id).collect();
        tracing::info!("Filtering rulse for cards");
        let rules = self.clone().find_and_filter_rules(&request, &card_type_ids).await?;
        tracing::info!("Using {} rules", rules.len());
        let ordered_cards = self.clone().get_card_order_from_rules(&cards, &rules, amount).await?;
        Ok(ordered_cards.into_iter().map(|card| card.to_owned()).collect())
    }

}

impl RuleService {
    #[cfg_attr(feature="trace-detail", tracing::instrument(skip_all))]
    pub fn new_with_services(
        category_service: Arc<dyn CategoryServiceTrait>,
        wallet_service: Arc<dyn WalletServiceTrait>
    ) -> Self {
        Self {
            category_service: category_service.clone(),
            rule_dao: Arc::new(RuleDao::new()),
            wallet_service: wallet_service.clone(),
        }
    }

    // TODO: this lifteime needs to be at class level
    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    pub async fn get_card_order_from_rules<'a>(self: Arc<Self>, cards: &'a Vec<WalletModel>, rules: &Vec<Rule>, amount_cents: i32) -> Result<Vec<&'a Wallet>, RuleError> {
        tracing::info!("Getting card order from rules");
        /*
        Order ever card in the users wallet based on the maximal reward amount we can get
        Precondition: expect rules to be pre-filtered
         */
        let mut max_reward_map: HashMap<i32, i32> = HashMap::new();
        for rule in rules {
            let reward_amount = rule.get_reward_amount_unitless(amount_cents);
            match max_reward_map.entry(rule.credit_card_id) {
                Entry::Vacant(e) => {e.insert(reward_amount);}
                Entry::Occupied(mut e) => {
                    if *e.get() < reward_amount {
                        e.insert(reward_amount);
                    }
                }
            }

        }
        let mut cards_only: Vec<&WalletModel> = cards.iter().map(|card_detail| card_detail).collect();
        tracing::info!("Sorting cards");
        cards_only.sort_by(|a_card, b_card| {
            let a_score = match max_reward_map.entry(a_card.credit_card_id) {
                Entry::Vacant(_) => 0,
                Entry::Occupied(e) => *e.get()
            };
            let b_score = match max_reward_map.entry(b_card.credit_card_id) {
                Entry::Vacant(_) => 0,
                Entry::Occupied(e) => *e.get()
            };
            b_score.cmp(&a_score)
        });
        Ok(cards_only)
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    pub async fn find_and_filter_rules(self: Arc<Self>, request: &AsaRequest, card_type_ids: &Vec<i32>) -> Result<Vec<Rule>, RuleError> {
        // TODO: remove direct call
        tracing::info!("Find and filter rules based on card types");
        let rules = self.rule_dao.clone().get_rules_for_card_ids(card_type_ids).await
            .map_err(|e| {
                tracing::error!("Error getting rules for card ids error={:?}", &e);
                RuleError::Unexpected(e.into())
            })?;
        let Some(merchant) = request.merchant.clone() else { return Ok(Vec::new()); };
        let Some(request_mcc) = merchant.mcc.clone() else { return Ok(Vec::new()); };
        let mcc_mapping = self.category_service.clone().get_mcc_mapping_by_mcc(&request_mcc).await
            .map_err(|e| RuleError::Unexpected(e.into()))?;
        let mut filtered_rules: Vec<Rule> = Vec::new();
        for rule in rules.into_iter() {
            if rule.is_valid() && self.clone().filter_rule_for_request(&rule, &request, &mcc_mapping).await {
                filtered_rules.push(rule)
            }
        }
        Ok( filtered_rules )
    }

    pub async fn filter_rule_for_request(self: Arc<Self>, rule: &Rule, asa_request: &AsaRequest, mapping: &MccMapping) -> bool {
        self.clone().filter_rule_by_merchant(rule, asa_request, mapping).await && self.clone().filter_rule_by_date(rule).await
    }

    pub async fn filter_rule_by_merchant(self: Arc<Self>, rule: &Rule, asa_request: &AsaRequest, mapping: &MccMapping) -> bool {
        let Some(merchant) = asa_request.merchant.clone() else { return false; };
        // TODO: this might need to be coupled with mcc
        if rule.merchant_name.is_some() {
            let Some(rule_merchant) = rule.merchant_name.as_ref() else { return false; };
            let Some(descriptor) = merchant.descriptor.clone() else { return false; };
            descriptor.to_lowercase() == *rule_merchant.to_lowercase()
        } else {
            //let Some(mcc) = rule.rule_category_id.as_ref() else { return false; };
            let Some(category_id) = rule.rule_category_id else { return false; };
            category_id == mapping.category_id
        }
    }

    pub async fn filter_rule_by_date(self: Arc<Self>, rule: &Rule) -> bool{
        let today = Utc::now().naive_utc().date();
        if rule.recurring_day_of_month.is_some() {
            tracing::info!("Filtering rule id {} by recurring day of month {:?}", rule.id, rule.recurring_day_of_month.as_ref());
            let Some(day_of_month) = rule.recurring_day_of_month.as_ref() else { return false; };
            let Ok(expected_date) = adjust_recurring_to_date(today, &day_of_month) else { return false; };
            expected_date == today
        } else if rule.start_date.is_none() && rule.end_date.is_none() && rule.recurring_day_of_month.is_none() {
            tracing::info!("Rule {} has no dates so is always valid", rule.id);
            true
        } else {
            tracing::info!("Filtering rule id {} by start {:?} end {:?}", rule.id, rule.start_date.as_ref(), rule.end_date.as_ref());
            let start_date = rule.start_date.unwrap();
            let end_date = rule.end_date.unwrap();
            start_date <= today
                && today <= end_date
        }
    }

}
