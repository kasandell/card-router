use std::collections::{hash_map::Entry, HashMap};

use chrono::Utc;

use crate::service_error::ServiceError;
use crate::asa::request::AsaRequest;
use crate::credit_card_type::entity::{CreditCard, CreditCardIssuer, CreditCardType};
use crate::user::entity::User;
use crate::util::date::adjust_recurring_to_date;
use crate::wallet::entity::Wallet;

use super::constant::DayOfMonth;
use super::entity::Rule;

pub type WalletDetail = (Wallet, CreditCard, CreditCardType, CreditCardIssuer);
pub struct RuleEngine {
}

#[mockall::automock]
pub trait RuleEngineTrait {
    fn order_user_cards_for_request(&self, request: AsaRequest, user: &User) -> Result<Vec<Wallet>, ServiceError>;
}

impl RuleEngineTrait for RuleEngine {
    fn order_user_cards_for_request(&self, request: AsaRequest, user: &User) -> Result<Vec<Wallet>, ServiceError> {
        /*
        Given an asa request, and a user, attempt charging against a user's wallet until we get a successful attempt
         */
        //wallet, credit_card, credit_card_type, credit_card_issuer
        let amount = request.amount.ok_or(ServiceError::new(400, "expect amount".to_string()))?;
        let cards = Wallet::find_all_for_user_with_card_info(user)?;
        let card_type_ids = cards.iter().map(|card_with_info| card_with_info.1.id).collect();
        let rules = RuleEngine::find_and_filter_rules(&request, &card_type_ids)?;
        info!("Using {} rules", rules.len());
        let ordered_cards = RuleEngine::get_card_order_from_rules(&cards, &rules, amount)?;
        Ok(ordered_cards.into_iter().map(|card| card.to_owned()).collect())
    }
}

impl RuleEngine {
    pub fn new() -> Self {
        Self {}
    }
    pub fn get_card_order_from_rules<'a>(cards: &'a Vec<WalletDetail>, rules: &Vec<Rule>, amount_cents: i32) -> Result<Vec<&'a Wallet>, ServiceError> {
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

    fn find_and_filter_rules(request: &AsaRequest, card_type_ids: &Vec<i32>) -> Result<Vec<Rule>, ServiceError> {
        Ok(
            Rule::get_rules_for_card_ids(card_type_ids)?
                .into_iter()
                .filter(|rule| rule.is_valid() && RuleEngine::filter_rule_for_request(&rule, &request))
                .collect()
        )
    }

    fn filter_rule_for_request(rule: &Rule, asa_request: &AsaRequest) -> bool {
        RuleEngine::filter_rule_by_merchant(rule, asa_request) && RuleEngine::filter_rule_by_date(rule)
    }

    fn filter_rule_by_merchant(rule: &Rule, asa_request: &AsaRequest) -> bool {
        let Some(merchant) = asa_request.merchant.clone() else { return false; };
        if rule.merchant_name.is_some() {
            let Some(rule_merchant) = rule.merchant_name.as_ref() else { return false; };
            let Some(descriptor) = merchant.descriptor.clone() else { return false; };
            descriptor.to_lowercase() == *rule_merchant.to_lowercase()
        } else {
            let Some(mcc) = rule.rule_mcc.as_ref() else { return false; };
            let Some(request_mcc) = merchant.mcc.clone() else { return false; };
            request_mcc == *mcc
        }
    }

    fn filter_rule_by_date(rule: &Rule) -> bool{
        let today = Utc::now().naive_utc().date();
        if rule.recurring_day_of_month.is_some() {
            info!("Filtering rule id {} by recurring day of month {:?}", rule.id, rule.recurring_day_of_month.as_ref());
            let Some(day_of_month) = rule.recurring_day_of_month.as_ref() else { return false; };
            let recur = DayOfMonth::from_str(
                &day_of_month
            );
            let expected_date = adjust_recurring_to_date(today, recur);
            expected_date == today
        } else if rule.start_date.is_none() && rule.end_date.is_none() && rule.recurring_day_of_month.is_none() {
            info!("Rule {} has no dates so is always valid", rule.id);
            true
        } else {
            info!("Filtering rule id {} by start {:?} end {:?}", rule.id, rule.start_date.as_ref(), rule.end_date.as_ref());
            let start_date = rule.start_date.unwrap();
            let end_date = rule.end_date.unwrap();
            start_date <= today 
            && today <= end_date
        }
    }
}
