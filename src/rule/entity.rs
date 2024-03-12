use crate::schema::rule;
use super::request::CreateRuleRequest;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::util::db;
use crate::data_error::DataError;
use crate::util::math::{
    get_cents_of_cashback,
    get_number_of_points
};
use super::constant::{DayOfMonth, RuleStatus};

#[derive(Insertable, Debug)]
#[diesel(table_name = rule)]
#[diesel(belongs_to(CreditCard))]
struct InsertableRule {
    pub credit_card_id: i32,
    // pub rule_mcc: Option<String>,
    pub rule_category_id: Option<i32>,
    pub merchant_name: Option<String>,
    pub points_multiplier: Option<i32>,
    pub cashback_percentage_bips: Option<i32>,
    pub recurring_day_of_month: Option<DayOfMonth>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub rule_status: RuleStatus
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Identifiable)]
#[diesel(table_name = rule)]
#[diesel(belongs_to(CreditCard))]
pub struct Rule {
    pub id: i32,
    pub public_id: Uuid,
    pub credit_card_id: i32,
    pub rule_category_id: Option<i32>,
    pub merchant_name: Option<String>,
    pub points_multiplier: Option<i32>,
    pub cashback_percentage_bips: Option<i32>,
    pub recurring_day_of_month: Option<DayOfMonth>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub rule_status: RuleStatus,
}

impl Rule {
    pub async fn create(new_rule: &CreateRuleRequest) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let rule = diesel::insert_into(rule::table)
            .values(InsertableRule::from(new_rule))
            .get_result(&mut conn).await?;
        Ok(rule)
    }

    pub async fn get_rules_for_card_ids(ids: &Vec<i32>) -> Result<Vec<Self>, DataError> {
        let mut conn = db::connection().await?;

        let rules = rule::table
            .filter(rule::credit_card_id.eq_any(ids))
            .load::<Rule>(&mut conn).await?;
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
        && self.is_valid_date_range()
    }

    fn is_active_rule(&self) -> bool {
        self.rule_status == RuleStatus::Active
    }

    fn is_valid_mcc_merchant_name(&self) -> bool {
        self.merchant_name.is_some() != self.rule_category_id.is_some()
    }

    fn is_valid_cashback_points(&self) -> bool {
        // rule can only be cashback or points
        self.points_multiplier.is_some() != self.cashback_percentage_bips.is_some()
    }

    fn is_valid_date_combo(&self) -> bool {
        //can either be a recurring date once a month, or have a start and end frame, or always active (no dates)
        if self.recurring_day_of_month.is_some() {
            self.start_date.is_none() && self.end_date.is_none()
        } else if self.start_date.is_some() {
            self.end_date.is_some() && self.recurring_day_of_month.is_none()
        } else if self.start_date.is_none() && self.end_date.is_none() && self.recurring_day_of_month.is_none() {
            true
        } else {
            false
        }
    }

    fn is_valid_date_range(&self) -> bool {
        if let Some(start) = self.start_date {
            if let Some(end) = self.end_date {
                return start <= end;
            }
        }
        true
    }

    #[cfg(test)]
    pub async fn delete(id: i32) -> Result<usize, DataError> {
        let mut conn = db::connection().await?;

        let res = diesel::delete(
                rule::table
                    .filter(rule::id.eq(id))
            )
            .execute(&mut conn).await?;
        Ok(res)
    }

    #[cfg(test)]
    pub async fn delete_self(&self) -> Result<usize, DataError> {
        Rule::delete(self.id).await
    }
}

impl From<&CreateRuleRequest> for InsertableRule {
    fn from(request: &CreateRuleRequest) -> Self {
        InsertableRule { 
            credit_card_id: request.credit_card_id,
            //rule_mcc: request.rule_mcc,
            rule_category_id: request.rule_category_id,
            merchant_name: match &request.merchant_name {
                Some(merchant_name) => Some(merchant_name.clone()),
                None => None
            },
            points_multiplier: request.points_multiplier,
            cashback_percentage_bips: request.cashback_percentage_bips,
            recurring_day_of_month: match &request.recurring_day_of_month {
                Some(dom) => Some(dom.clone()),
                None => None
            },
            start_date: match request.start_date {
                Some(start_date) => Some(start_date.clone()),
                None => None
            },
            end_date: match request.end_date {
                Some(end_date) => Some(end_date.clone()),
                None => None
            },
            rule_status: RuleStatus::Active
        }
    }
}