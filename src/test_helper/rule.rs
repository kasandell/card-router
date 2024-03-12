use uuid::Uuid;
use crate::rule::constant::RuleStatus;
use crate::rule::entity::Rule;

pub fn create_mock_rule_dateless_mcc_points(
    id: i32,
    credit_card_id: i32,
    points_multiplier: i32
) -> Rule {
    Rule {
        id: id,
        public_id: Uuid::new_v4(),
        credit_card_id: credit_card_id,
        rule_category_id: Some(1),
        merchant_name: None,
        points_multiplier: Some(points_multiplier),
        cashback_percentage_bips: None,
        recurring_day_of_month: None,
        start_date: None,
        end_date: None,
        rule_status: RuleStatus::Active
    }
}

pub fn create_mock_rule_dateless_mcc_cashback(
    id: i32,
    credit_card_id: i32,
    cashback_percentage_bips: i32
) -> Rule {
    Rule {
        id: id,
        public_id: Uuid::new_v4(),
        credit_card_id: credit_card_id,
        rule_category_id: Some(1),
        merchant_name: None,
        points_multiplier: None,
        cashback_percentage_bips: Some(cashback_percentage_bips),
        recurring_day_of_month: None,
        start_date: None,
        end_date: None,
        rule_status: RuleStatus::Active
    }
}