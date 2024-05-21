#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::category::model::{CategoryModel, MccMappingModel};
    use crate::rule::constant::DayOfMonth;
    use crate::rule::request::CreateRuleRequest;
    use crate::rule::service::{
        RuleService,
        RuleServiceTrait,
    };
    use crate::wallet::model::{WalletModel as Wallet, WalletModel, WalletModelWithRule};
    use crate::rule::entity::Rule;
    use crate::asa::request::create_example_asa;
    use chrono::Utc;
    use crate::rule::entity::{create_mock_rule_dateless_mcc_cashback, create_mock_rule_dateless_mcc_points};
    use crate::test_helper::wallet::create_mock_wallet_with_args;
    use actix_web::test;
    use mockall::predicate::eq;
    use crate::category::service::MockCategoryServiceTrait;
    use crate::credit_card_type::constant::CreditCardTypeEnum;
    use crate::test_helper::user::create_mock_user;
    use crate::wallet::constant::WalletStatus;
    use crate::wallet::service::MockWalletServiceTrait;

    const RULE_CATEGORY: i32 = 1;

    #[test]
    async fn test_filter_rules() {
        crate::test_helper::general::init();
        let amount_cents = 30000;
        let mut cards: Vec<WalletModelWithRule> = Vec::new();

        let category_service = MockCategoryServiceTrait::new();
        let wallet_service = MockWalletServiceTrait::new();
        let rule_engine = Arc::new(RuleService::new_with_services(
            Arc::new(category_service),
            Arc::new(wallet_service)
        ));
        cards.push(create_mock_wallet_with_args(1, 1, 1).into());

       cards.push(create_mock_wallet_with_args(2, 1, 2).into());

        let mut rules: Vec<Rule> = Vec::new();
        rules.push(
            create_mock_rule_dateless_mcc_points(1, 1, 2)
        );
        rules.push(
            create_mock_rule_dateless_mcc_points(2, 2, 3)
        );

        let wallet_returned = rule_engine.order_cards_from_rules_and_attach_rule_id_in_place(&mut cards, &rules, amount_cents).await.expect("wallet should come back");
        assert_eq!(wallet_returned[0].credit_card_id, 2);
        assert_eq!(wallet_returned[1].credit_card_id, 1);
        assert_eq!(wallet_returned.len(), 2);
    }


    #[test]
    async fn test_filter_rules_cashback_and_points() {
        crate::test_helper::general::init();
        let amount_cents = 30000;
        let mut cards: Vec<WalletModelWithRule> = Vec::new();
        cards.push(create_mock_wallet_with_args(1, 1, 1).into());

       cards.push(create_mock_wallet_with_args(2, 1, 2).into());

        let mut rules: Vec<Rule> = Vec::new();
        rules.push(
            create_mock_rule_dateless_mcc_points(1, 1, 2) // 2x point multiple
        );
        rules.push(
            create_mock_rule_dateless_mcc_cashback(2, 2, 250) // 2.5% cashback
        );

        let category_service = MockCategoryServiceTrait::new();
        let wallet_service = MockWalletServiceTrait::new();
        let rule_engine = Arc::new(RuleService::new_with_services(
            Arc::new(category_service),
            Arc::new(wallet_service)
        ));

        let wallet_returned = rule_engine.order_cards_from_rules_and_attach_rule_id_in_place(&mut cards, &rules, amount_cents).await.expect("wallet should come back");
        assert_eq!(wallet_returned[0].credit_card_id, 2);
        assert_eq!(wallet_returned[1].credit_card_id, 1);
        assert_eq!(wallet_returned.len(), 2);
    }

    // TODO: disabled while we can't get category to insert properly
    async fn test_order_user_cards_for_request() {
        crate::test_helper::general::init();
        let user = create_mock_user();
        let payment_method_id_1 = "s_1234";
        let payment_method_id_2 = "s_1235";
        let card_1_id = 1;
        let card_2_id = 2;
        let rule_mcc = "0000";


        let cat = CategoryModel {
            id: 1,
            public_id: Default::default(),
            name: "Test".to_string(),
        };

        let mcc_mapping = MccMappingModel {
            id: 1,
            public_id: Default::default(),
            mcc_code: rule_mcc.to_string(),
            category_id: 1,
        };

        let card_1 = WalletModel {
            id: card_1_id,
            public_id: Default::default(),
            user_id: user.id,
            payment_method_id: payment_method_id_1.to_string(),
            created_at: Default::default(),
            credit_card_id: CreditCardTypeEnum::ChaseSapphirePreferred.into(),
            wallet_card_attempt_id: 1,
            status: WalletStatus::Active,
        };
        let card_2 = WalletModel {
            id: card_2_id,
            public_id: Default::default(),
            user_id: user.id,
            payment_method_id: payment_method_id_2.to_string(),
            created_at: Default::default(),
            credit_card_id: CreditCardTypeEnum::ChaseSapphireReserve.into(),
            wallet_card_attempt_id: 1,
            status: WalletStatus::Active,
        };

        let should_be_filtered_rule = Rule::create(
            &CreateRuleRequest {
                credit_card_id: 1,
                rule_category_id: Some(RULE_CATEGORY),
                merchant_name: None,
                points_multiplier: Some(1000),
                cashback_percentage_bips: None,
                recurring_day_of_month: Some(DayOfMonth::First),
                start_date: Some(Utc::now().naive_utc().date()),
                end_date: None,
            }
        ).await.expect("rule should be created");



        let card_1_rule = Rule::create(
            &CreateRuleRequest {
                credit_card_id: 1,
                rule_category_id: Some(RULE_CATEGORY),
                merchant_name: None,
                points_multiplier: Some(2),
                cashback_percentage_bips: None,
                recurring_day_of_month: None,
                start_date: None,
                end_date: None,
            }
        ).await.expect("rule should be created");

        let card_2_rule = Rule::create(
            &CreateRuleRequest {
                credit_card_id: 2,
                rule_category_id: Some(RULE_CATEGORY),
                merchant_name: None,
                points_multiplier: Some(5),
                cashback_percentage_bips: None,
                recurring_day_of_month: None,
                start_date: None,
                end_date: None,
            }
        ).await.expect("rule should be created");

        let mut category_service = MockCategoryServiceTrait::new();
        let mut wallet_service = MockWalletServiceTrait::new();
        wallet_service.expect_find_all_for_user()
            .times(1)
            .return_once(move |_| Ok(vec![card_1, card_2]));

        category_service.expect_get_mcc_mapping_by_mcc()
            .with(eq(rule_mcc))
            .times(1)
            .return_once(move |_| Ok(mcc_mapping));

        let rule_engine = Arc::new(RuleService::new_with_services(
            Arc::new(category_service),
            Arc::new(wallet_service)
        ));
        let cards = rule_engine.clone().order_user_cards_for_request(
            &create_example_asa(30000, "0000".to_string()),
            &user
        ).await.expect("should get cards");
        assert_eq!(cards.len(), 2);
        assert_eq!(cards[0].credit_card_id, 2);
        assert_eq!(cards[0].id, card_2_id);
        assert_eq!(cards[1].credit_card_id, 1);
        assert_eq!(cards[1].id, card_1_id);
    }
}