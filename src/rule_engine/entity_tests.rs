#[cfg(test)]
mod entity_tests {
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
    async fn test_create_rule() {
        crate::test::init();
        let user = initialize_user();
        let mcc = "7184";
        let category = Category::create(
            InsertableCategory { name: "Random".to_string() }
        ).expect("should create");
        let mcc_mapping = MccMapping::create(
            InsertableMccMapping { 
                mcc_code: mcc.to_string(),
                category_id: category.id
            }
        ).expect("should create");
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
        let rule = Rule::create(rule_to_create).expect("Rule creation should not fail");
        assert_eq!(credit_card_id, rule.credit_card_id);
        assert_eq!(mcc, rule.rule_mcc.expect("expect rule mcc"));
        assert_eq!(points_multiplier, rule.points_multiplier);
        assert_eq!(RuleStatus::ACTIVE.as_str(), rule.rule_status);
        assert!(rule.merchant_name.is_none());
        assert!(rule.cashback_percentage_bips.is_none());
        assert!(rule.recurring_day_of_month.is_none());
        assert!(rule.start_date.is_none());
        assert!(rule.end_date.is_none());
        user.delete_self().expect("should delete");
        Rule::delete(rule.id).expect("should delete");
        mcc_mapping.delete_self().expect("should delete");
        category.delete_self().expect("should delete");

    }
}