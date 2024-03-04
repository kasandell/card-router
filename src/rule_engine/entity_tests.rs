#[cfg(test)]
mod entity_tests {
    use std::sync::Arc;
    use chrono::{Utc, Duration};
    use uuid::Uuid;
    use crate::rule_engine::constant::DayOfMonth;
    use crate::test_helper::initialize_user;
    use crate::rule_engine::{
        request::CreateRuleRequest,
        entity::Rule,
        constant::RuleStatus
    };
    use crate::category::entity::{
        Category,
        InsertableCategory,
        MccMapping,
        InsertableMccMapping
    };

    #[actix_web::test]
    async fn test_create_rule_in_db() {
        crate::test::init();
        let user = initialize_user().await;
        let mcc = "7184";
        let category = Category::create(
            InsertableCategory { name: "Random".to_string() }
        ).await.expect("should create");
        let mcc_mapping = MccMapping::create(
            InsertableMccMapping { 
                mcc_code: mcc.to_string(),
                category_id: category.id
            }
        ).await.expect("should create");
        let points_multiplier = Some(2);
        let credit_card_id = 1;
        let rule_to_create = CreateRuleRequest {
            credit_card_id: credit_card_id,
            rule_mcc: Some(mcc.to_string()),
            points_multiplier: points_multiplier,
            merchant_name: None,
            cashback_percentage_bips: None,
            recurring_day_of_month: None,
            start_date: None,
            end_date: None
        };
        let rule = Rule::create(rule_to_create).await.expect("Should create");
        assert_eq!(credit_card_id, rule.credit_card_id);
        assert!(rule.is_valid());
        assert_eq!(mcc, rule.rule_mcc.expect("expect rule mcc"));
        assert_eq!(points_multiplier, rule.points_multiplier);
        assert_eq!(RuleStatus::ACTIVE.as_str(), rule.rule_status);
        assert!(rule.merchant_name.is_none());
        assert!(rule.cashback_percentage_bips.is_none());
        assert!(rule.recurring_day_of_month.is_none());
        assert!(rule.start_date.is_none());
        assert!(rule.end_date.is_none());
        user.delete_self().await.expect("should delete");
        Rule::delete(rule.id).await.expect("should delete");
        mcc_mapping.delete_self().await.expect("should delete");
        category.delete_self().await.expect("should delete");
    }

    #[actix_web::test]
    async fn test_rule_invalid_start_and_recur() {
        crate::test::init();
        let points_multiplier = Some(2);
        let credit_card_id = 1;
        let mcc = "7184";
        let date = Some(Utc::now().date_naive());
        let recurring_day_of_month = Some(DayOfMonth::FIRST.as_str());
        let rule = Rule {
            id: 1,
            public_id: Uuid::new_v4(),
            credit_card_id: credit_card_id,
            rule_mcc: Some(mcc.to_string()),
            points_multiplier: points_multiplier,
            merchant_name: None,
            cashback_percentage_bips: None,
            recurring_day_of_month: recurring_day_of_month.clone(),
            start_date: date,
            end_date: None,
            rule_status: RuleStatus::ACTIVE.as_str()
        };
        assert!(!rule.is_valid());
        assert_eq!(RuleStatus::ACTIVE.as_str(), rule.rule_status);
        assert_eq!(rule.recurring_day_of_month, recurring_day_of_month.clone());
        assert_eq!(rule.start_date, date);
        assert!(rule.end_date.is_none());
    }

    #[actix_web::test]
    async fn test_rule_invalid_start_no_end() {
        crate::test::init();
        let points_multiplier = Some(2);
        let credit_card_id = 1;
        let mcc = "7184";
        let date = Some(Utc::now().date_naive());
        let rule = Rule {
            id: 1,
            public_id: Uuid::new_v4(),
            credit_card_id: credit_card_id,
            rule_mcc: Some(mcc.to_string()),
            points_multiplier: points_multiplier,
            merchant_name: None,
            cashback_percentage_bips: None,
            recurring_day_of_month: None,
            start_date: date,
            end_date: None,
            rule_status: RuleStatus::ACTIVE.as_str()
        };
        assert!(!rule.is_valid());
        assert_eq!(rule.start_date, date);
        assert!(rule.end_date.is_none());
    }

    #[actix_web::test]
    async fn test_rule_invalid_start_gt_end() {
        crate::test::init();
        let points_multiplier = Some(2);
        let credit_card_id = 1;
        let mcc = "7184";
        let init_date = Utc::now().date_naive();
        let start_date = Some(init_date.clone());
        let end_date = Some(init_date.clone() - Duration::days(1));
        let rule = Rule {
            id: 1,
            public_id: Uuid::new_v4(),
            credit_card_id: credit_card_id,
            rule_mcc: Some(mcc.to_string()),
            points_multiplier: points_multiplier,
            merchant_name: None,
            cashback_percentage_bips: None,
            recurring_day_of_month: None,
            start_date: start_date,
            end_date: end_date,
            rule_status: RuleStatus::ACTIVE.as_str()
        };
        assert!(!rule.is_valid());
        assert_eq!(rule.start_date, start_date);
        assert_eq!(rule.end_date, end_date);
    }

    #[actix_web::test]
    async fn test_rule_invalid_mcc_merchant_none() {
        crate::test::init();
        let points_multiplier = Some(2);
        let credit_card_id = 1;
        let rule = Rule {
            id: 1,
            public_id: Uuid::new_v4(),
            credit_card_id: credit_card_id,
            rule_mcc: None,
            points_multiplier: points_multiplier,
            merchant_name: None,
            cashback_percentage_bips: None,
            recurring_day_of_month: None,
            start_date: None,
            end_date: None,
            rule_status: RuleStatus::ACTIVE.as_str()
        };
        assert!(!rule.is_valid());
        assert!(rule.rule_mcc.is_none());
        assert!(rule.merchant_name.is_none());
    }

    #[actix_web::test]
    async fn test_rule_invalid_mcc_merchant_both_some() {
        crate::test::init();
        let points_multiplier = Some(2);
        let credit_card_id = 1;
        let mcc = "7184";
        let merchant_name = "Kyle's Merchant";
        let rule = Rule {
            id: 1,
            public_id: Uuid::new_v4(),
            credit_card_id: credit_card_id,
            rule_mcc: Some(mcc.to_string()),
            points_multiplier: points_multiplier,
            merchant_name: Some(merchant_name.to_string()),
            cashback_percentage_bips: None,
            recurring_day_of_month: None,
            start_date: None,
            end_date: None,
            rule_status: RuleStatus::ACTIVE.as_str()
        };
        assert!(!rule.is_valid());
        assert_eq!(rule.rule_mcc, Some(mcc.to_string()));
        assert_eq!(rule.merchant_name, Some(merchant_name.to_string()));
    }

    #[actix_web::test]
    async fn test_rule_invalid_no_reward() {
        crate::test::init();
        let credit_card_id = 1;
        let mcc = "7184";
        let rule = Rule {
            id: 1,
            public_id: Uuid::new_v4(),
            credit_card_id: credit_card_id,
            rule_mcc: Some(mcc.to_string()),
            points_multiplier: None,
            merchant_name: None,
            cashback_percentage_bips: None,
            recurring_day_of_month: None,
            start_date: None,
            end_date: None,
            rule_status: RuleStatus::ACTIVE.as_str()
        };
        assert!(!rule.is_valid());
        assert!(rule.points_multiplier.is_none());
        assert!(rule.cashback_percentage_bips.is_none());
    }

    #[actix_web::test]
    async fn test_rule_invalid_both_reward() {
        crate::test::init();
        let points_multiplier = Some(2);
        let cashback_percentage_bips = Some(500);
        let credit_card_id = 1;
        let mcc = "7184";
        let rule = Rule {
            id: 1,
            public_id: Uuid::new_v4(),
            credit_card_id: credit_card_id,
            rule_mcc: Some(mcc.to_string()),
            points_multiplier: points_multiplier,
            merchant_name: None,
            cashback_percentage_bips: cashback_percentage_bips,
            recurring_day_of_month: None,
            start_date: None,
            end_date: None,
            rule_status: RuleStatus::ACTIVE.as_str()
        };
        assert!(!rule.is_valid());
        assert_eq!(rule.points_multiplier, points_multiplier);
        assert_eq!(rule.cashback_percentage_bips, cashback_percentage_bips);
    }
}