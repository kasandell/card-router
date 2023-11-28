use crate::schema::rule;
use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::util::db;
use crate::api_error::ApiError;

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Identifiable)]
#[diesel(table_name = rule)]
#[diesel(belongs_to(CreditCard))]
pub struct Rule {
    pub id: i32,
    pub public_id: Uuid,
    pub credit_card_id: i32,
    pub rule_mcc: Option<String>,
    pub merchant_name: Option<String>,
    pub recurring_day_of_month: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub rule_status: String,
}

impl Rule {
    fn get_rules_for_card_ids(ids: Vec<i32>) -> Result<Vec<Self>, ApiError> {
        let mut conn = db::connection()?;

        let rules = rule::table
            .filter(rule::credit_card_id.eq_any(ids))
            .load::<Rule>(&mut conn)?;
        Ok(rules)
    }
}