use std::cmp::Ordering;

use super::constant::DayOfMonth;
use super::entity::Rule;
use crate::asa_request::entity::AsaRequest;
use crate::user::entity::User;
use crate::wallet::entity::Wallet;
use crate::credit_card_type::entity::{CreditCard, CreditCardIssuer, CreditCardType};
use crate::api_error::ApiError;
use crate::util::date::adjust_recurring_to_date;
use chrono::Utc;
use std::collections::{HashMap, hash_map::Entry};


type WalletDetail = (Wallet, CreditCard, CreditCardType, CreditCardIssuer);
struct Engine {
}

impl Engine {
    fn charge_in_order(request: AsaRequest, user: User) -> Result<(), ApiError> {
        /*
        Given an asa request, and a user, attempt charging against a user's wallet until we get a successful attempt
         */
        //wallet, credit_card, credit_card_type, credit_card_issuer
        let cards = Wallet::find_all_for_user_with_card_info(user)?;
        let card_type_ids = cards.iter().map(|card_with_info| card_with_info.1.id).collect();
        let mut rules = Engine::find_and_filter_rules(&request, &card_type_ids)?;
        info!("Using {} rules", rules.len());
        let ordered_cards = Engine::get_card_order_from_rules(&cards, &rules, request.amount)?;

        Ok(())
    }

    fn get_card_order_from_rules<'a>(cards: &'a Vec<WalletDetail>, ordered_rules: &Vec<Rule>, amount_cents: i32) -> Result<Vec<&'a Wallet>, ApiError> {
        /*
        Order ever card in the users wallet based on the maximal reward amount we can get
         */
        let mut max_reward_map: HashMap<i32, i32> = HashMap::new();
        for rule in ordered_rules {
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
            if a_score > b_score {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
        Ok(cards_only)
        /*
        // join cards to the rules in order, then filter to unique cards
        let mut card_id_map: HashMap<i32, &Wallet> = HashMap::new();
        for card in cards {
            let key = card.2.id;
            match card_id_map.entry(key) {
                Entry::Vacant(e) => { e.insert(&card.0); },
                Entry::Occupied(mut e) => { continue; }
            }
        }
        let mut wallet: Vec<&Wallet> = ordered_rules
                    .iter()
                    //get the card to use based on this rule
                    .map(|rule| card_id_map.get(&rule.credit_card_id))
                    //remove any None
                    .filter_map(|rule| rule)
                    .map(|rule| *rule)
                    .collect();
        dedup_wallet(&mut wallet);
        Ok(wallet)
        */
    }

    fn find_and_filter_rules(request: &AsaRequest, card_type_ids: &Vec<i32>) -> Result<Vec<Rule>, ApiError> {
        Ok(
            Rule::get_rules_for_card_ids(card_type_ids)?
                .into_iter()
                .filter(|rule| rule.is_valid() && Engine::filter_rule_for_request(&rule, &request))
                .collect()
        )
    }



    fn filter_rule_for_request(rule: &Rule, asa_request: &AsaRequest) -> bool {
        Engine::filter_rule_by_merchant(rule, asa_request) && Engine::filter_rule_by_date(rule, asa_request)
    }

    fn filter_rule_by_merchant(rule: &Rule, asa_request: &AsaRequest) -> bool {
        if rule.merchant_name.is_some() {
            let Some(merchant) = rule.merchant_name.as_ref() else { return false; };
            asa_request.merchant.descriptor.to_lowercase() == *merchant.to_lowercase()
        } else {
            let Some(mcc) = rule.rule_mcc.as_ref() else { return false; };
            asa_request.merchant.mcc == *mcc
        }
    }

    fn filter_rule_by_date(rule: &Rule, asa_request: &AsaRequest) -> bool{
        let today = Utc::now().naive_utc().date();
        if rule.recurring_day_of_month.is_some() {
            let Some(day_of_month) = rule.recurring_day_of_month.as_ref() else { return false; };
            let recur = DayOfMonth::from_str(
                &day_of_month
            );
            let expected_date = adjust_recurring_to_date(today, recur);
            expected_date == today
        } else {
            let start_date = rule.start_date.unwrap();
            let end_date = rule.end_date.unwrap();
            start_date <= today 
            && today <= end_date
        }
    }
}