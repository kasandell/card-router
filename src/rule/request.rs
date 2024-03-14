use chrono::NaiveDate;
use crate::rule::constant::DayOfMonth;

#[derive(Debug)]
pub struct CreateRuleRequest {
    pub credit_card_id: i32,
    //pub rule_mcc: Option<String>,
    pub rule_category_id: Option<i32>,
    pub merchant_name: Option<String>,
    pub points_multiplier: Option<i32>,
    pub cashback_percentage_bips: Option<i32>,
    pub recurring_day_of_month: Option<DayOfMonth>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}