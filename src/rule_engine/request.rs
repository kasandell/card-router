use chrono::NaiveDate;
pub struct CreateRuleRequest {
    pub credit_card_id: i32,
    pub rule_mcc: Option<String>,
    pub merchant_name: Option<String>,
    pub points_multiplier: Option<i32>,
    pub cashback_percentage_bips: Option<i32>,
    pub recurring_day_of_month: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}