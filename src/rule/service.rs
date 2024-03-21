use std::collections::{hash_map::Entry, HashMap};
use std::fmt::Formatter;
use std::sync::Arc;
use std::time::Instant;
use async_trait::async_trait;

use chrono::Utc;

use crate::error::error::ServiceError;
use crate::asa::request::AsaRequest;
use crate::category::dao::{MccMappingDao, MccMappingDaoTrait};
use crate::category::entity::MccMapping;
use crate::credit_card_type::entity::{CreditCard, CreditCardIssuer, CreditCardType};

use crate::user::entity::User;
use crate::util::date::adjust_recurring_to_date;
use crate::wallet::entity::Wallet;

use super::constant::DayOfMonth;
use super::entity::Rule;

pub type WalletDetail = (Wallet, CreditCard, CreditCardType, CreditCardIssuer);
pub struct RuleService {
    mcc_mapping_dao: Arc<dyn MccMappingDaoTrait>,
}


#[mockall::automock]
#[async_trait(?Send)]
pub trait RuleServiceTrait {
    async fn order_user_cards_for_request(self: Arc<Self>, request: &AsaRequest, user: &User) -> Result<Vec<Wallet>, ServiceError>;
}




#[async_trait(?Send)]
impl RuleServiceTrait for RuleService {

    #[tracing::instrument(skip_all)]
    async fn order_user_cards_for_request(self: Arc<Self>, request: &AsaRequest, user: &User) -> Result<Vec<Wallet>, ServiceError> {
        /*
        Given an asa request, and a user, attempt charging against a user's wallet until we get a successful attempt
         */
        //wallet, credit_card, credit_card_type, credit_card_issuer
        let amount = request.amount.ok_or(ServiceError::Format(Box::new("expect amount")))?;
        // TODO: not from dao
        let mut start = Instant::now();
        let cards = Wallet::find_all_for_user_with_card_info(user).await?;
        tracing::info!("Find cards for user with info took {:?}", start.elapsed());
        start = Instant::now();
        let card_type_ids = cards.iter().map(|card_with_info| card_with_info.1.id).collect();
        tracing::info!("card type id get took {:?}", start.elapsed());
        start = Instant::now();
        let rules = self.clone().find_and_filter_rules(&request, &card_type_ids).await?;
        tracing::info!("find and filter rules took {:?}", start.elapsed());
        tracing::info!("Using {} rules", rules.len());
        tracing::info!("Using {} rules", rules.len());
        start = Instant::now();
        let ordered_cards = self.clone().get_card_order_from_rules(&cards, &rules, amount).await?;
        tracing::info!("Order cards took {:?}", start.elapsed());
        Ok(ordered_cards.into_iter().map(|card| card.to_owned()).collect())
    }

}

// TODO: create as a self reference?

impl RuleService {
    #[tracing::instrument]
    pub fn new() -> Self {
        Self {
            mcc_mapping_dao: Arc::new(MccMappingDao::new())
        }
    }

    #[tracing::instrument(skip_all)]
    pub fn new_with_services(
        mcc_mapping_dao: Arc<dyn MccMappingDaoTrait>
    ) -> Self {
        Self {
            mcc_mapping_dao
        }
    }


    // TODO: this lifteime needs to be at class level
    #[tracing::instrument(skip(self))]
    pub async fn get_card_order_from_rules<'a>(self: Arc<Self>, cards: &'a Vec<WalletDetail>, rules: &Vec<Rule>, amount_cents: i32) -> Result<Vec<&'a Wallet>, ServiceError> {
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
        let mut cards_only: Vec<&Wallet> = cards.iter().map(|card_detail| &card_detail.0).collect();
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

    #[tracing::instrument(skip(self))]
    pub async fn find_and_filter_rules(self: Arc<Self>, request: &AsaRequest, card_type_ids: &Vec<i32>) -> Result<Vec<Rule>, ServiceError> {
        let rules = Rule::get_rules_for_card_ids(card_type_ids).await?;
        let Some(merchant) = request.merchant.clone() else { return Ok(Vec::new()); };
        let Some(request_mcc) = merchant.mcc.clone() else { return Ok(Vec::new()); };
        let mcc_mapping = self.mcc_mapping_dao.clone().get_by_mcc(&request_mcc).await?;
        let mut filtered_rules: Vec<Rule> = Vec::new();
        for rule in rules.into_iter() {
            if rule.is_valid() && self.clone().filter_rule_for_request(&rule, &request, &mcc_mapping).await {
                filtered_rules.push(rule)
            }
        }
        Ok( filtered_rules )
    }

    // TODO: async?
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
            // TODO: we need to join this earlier
            category_id == mapping.category_id
        }
    }

    pub async fn filter_rule_by_date(self: Arc<Self>, rule: &Rule) -> bool{
        let today = Utc::now().naive_utc().date();
        if rule.recurring_day_of_month.is_some() {
            tracing::info!("Filtering rule id {} by recurring day of month {:?}", rule.id, rule.recurring_day_of_month.as_ref());
            let Some(day_of_month) = rule.recurring_day_of_month.as_ref() else { return false; };
            let expected_date = adjust_recurring_to_date(today, &day_of_month);
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
