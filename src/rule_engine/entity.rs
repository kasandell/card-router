use crate::schema::rule::{points_multiplier, cashback_percentage_bips};
use crate::schema::{
    rule
};
use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::util::db;
use crate::api_error::ApiError;
use crate::util::math::{
    get_cents_of_cashback,
    get_number_of_points
};
use crate::credit_card_type::entity::CreditCard;

use super::constant::RuleStatus;

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Identifiable)]
#[diesel(table_name = rule)]
#[diesel(belongs_to(CreditCard))]
pub struct Rule {
    pub id: i32,
    pub public_id: Uuid,
    pub credit_card_id: i32,
    pub rule_mcc: Option<String>,
    pub merchant_name: Option<String>,
    pub points_multiplier: Option<i32>,
    pub cashback_percentage_bips: Option<i32>,
    pub recurring_day_of_month: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub rule_status: String,
}

impl Rule {
    pub fn get_rules_for_card_ids(ids: Vec<i32>) -> Result<Vec<Self>, ApiError> {
        let mut conn = db::connection()?;

        let rules = rule::table
            .filter(rule::credit_card_id.eq_any(ids))
            .load::<Rule>(&mut conn)?;
        Ok(rules)
    }

    pub fn get_reward_amount_unitless(&self, amount_cents: i32) -> i32 {
        //assumes points are 1 cent so we use either cashback cents or points depending on what we have
        if let Some(pm) = self.points_multiplier {
            get_number_of_points(amount_cents, pm)
        } else if let Some(cpb)  = self.cashback_percentage_bips {
            get_cents_of_cashback(amount_cents, cpb)
        } else {
            0
        }
    }

    pub fn is_valid(&self) -> bool {
        self.is_active_rule()
        && self.is_valid_mcc_merchant_name()
        && self.is_valid_cashback_points()
        && self.is_valid_date_combo()
    }

    fn is_active_rule(&self) -> bool {
        RuleStatus::from_str(&self.rule_status) == RuleStatus::VALID
    }

    fn is_valid_mcc_merchant_name(&self) -> bool {
        self.merchant_name.is_some() != self.rule_mcc.is_some()
    }

    fn is_valid_cashback_points(&self) -> bool {
        // rule can only be cashback or points
        self.points_multiplier.is_some() != self.cashback_percentage_bips.is_some()
    }

    fn is_valid_date_combo(&self) -> bool {
        //can either be a recurring date once a month, or have a start and end frame
        if self.recurring_day_of_month.is_some() {
            self.start_date.is_none() & self.end_date.is_none()
        } else if self.start_date.is_some() {
            self.end_date.is_some() & self.recurring_day_of_month.is_none()
        } else {
            false
        }
    }
}