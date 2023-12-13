#[cfg(test)]
mod tests {
    use crate::category::entity::{Category, InsertableCategory, MccMapping, InsertableMccMapping};
    use crate::credit_card_type::entity::{CreditCardType, CreditCardIssuer, CreditCard};
    use crate::rule_engine::constant::DayOfMonth;
    use crate::rule_engine::request::CreateRuleRequest;
    use crate::test_helper::initialize_user;
    use crate::rule_engine::engine::{
        RuleEngine,
        WalletDetail
    };
    use crate::wallet::entity::{Wallet, NewCard, WalletCardAttempt, InsertableCardAttempt};
    use crate::rule_engine::entity::Rule;
    use crate::asa_request::entity::create_example_asa;
    use chrono::Utc;

    #[actix_web::test]
    async fn test_filter_rules() {
        crate::test::init();
        let amount_cents = 30000;
        let mut cards: Vec<WalletDetail> = Vec::new();
        cards.push(
            (
                Wallet::create_test_wallet(1, 1, 1),
                CreditCard::create_test_credit_card(1, "Sapphire".to_string(), 1, 1),
                CreditCardType::create_test_credit_card_type(1, "Chase".to_string()),
                CreditCardIssuer::create_test_credit_card_issuer(1, "Visa".to_string())
            )
       );

       cards.push(
            (
                Wallet::create_test_wallet(2, 1, 2),
                CreditCard::create_test_credit_card(2, "Bilt".to_string(), 1, 1),
                CreditCardType::create_test_credit_card_type(2, "Bilt".to_string()),
                CreditCardIssuer::create_test_credit_card_issuer(1, "MasterCard".to_string())
            )
        );

        let mut rules: Vec<Rule> = Vec::new();
        rules.push(
            Rule::create_test_rule_dateless_mcc_points(1, 1, "7184".to_string(), 2)
        );
        rules.push(
            Rule::create_test_rule_dateless_mcc_points(2, 2, "7184".to_string(), 3)
        );

        let wallet_returned = RuleEngine::get_card_order_from_rules(&cards, &rules, amount_cents).expect("wallet should come back");
        assert_eq!(wallet_returned[0].credit_card_id, 2);
        assert_eq!(wallet_returned[1].credit_card_id, 1);
        assert_eq!(wallet_returned.len(), 2);
        println!("{:?}", wallet_returned);
    }


    #[actix_web::test]
    async fn test_filter_rules_cashback_and_points() {
        crate::test::init();
        let amount_cents = 30000;
        let mut cards: Vec<WalletDetail> = Vec::new();
        cards.push(
            (
                Wallet::create_test_wallet(1, 1, 1),
                CreditCard::create_test_credit_card(1, "Sapphire".to_string(), 1, 1),
                CreditCardType::create_test_credit_card_type(1, "Chase".to_string()),
                CreditCardIssuer::create_test_credit_card_issuer(1, "Visa".to_string())
            )
       );

       cards.push(
            (
                Wallet::create_test_wallet(2, 1, 2),
                CreditCard::create_test_credit_card(2, "Bilt".to_string(), 1, 1),
                CreditCardType::create_test_credit_card_type(2, "Bilt".to_string()),
                CreditCardIssuer::create_test_credit_card_issuer(1, "MasterCard".to_string())
            )
        );

        let mut rules: Vec<Rule> = Vec::new();
        rules.push(
            Rule::create_test_rule_dateless_mcc_points(1, 1, "7184".to_string(), 2) // 2x point multiple
        );
        rules.push(
            Rule::create_test_rule_dateless_mcc_cashback(2, 2, "7184".to_string(), 250) // 2.5% cashback
        );

        let wallet_returned = RuleEngine::get_card_order_from_rules(&cards, &rules, amount_cents).expect("wallet should come back");
        assert_eq!(wallet_returned[0].credit_card_id, 2);
        assert_eq!(wallet_returned[1].credit_card_id, 1);
        assert_eq!(wallet_returned.len(), 2);
        println!("{:?}", wallet_returned);
    }

    #[actix_web::test]
    async fn test_order_user_cards_for_request() {
        crate::test::init();
        let user = initialize_user();
        let payment_method_id_1 = "s_1234";
        let payment_method_id_2 = "s_1235";
        let rule_mcc = "0000";

        let att1 = WalletCardAttempt::insert(
            InsertableCardAttempt {
                user_id: user.id,
                credit_card_id: 1,
                expected_reference_id: "a".to_string()
            }
        ).expect("expect attempt to create");

        let att2 = WalletCardAttempt::insert(
            InsertableCardAttempt {
                user_id: user.id,
                credit_card_id: 2,
                expected_reference_id: "b".to_string()
            }
        ).expect("expect attempt to create");

        let category = Category::create(
            InsertableCategory {
                name: "Test".to_string()
            }
        ).expect("should create category");
        let mcc_mapping = MccMapping::create(
            InsertableMccMapping {
                mcc_code: rule_mcc.to_string(),
                category_id: category.id
            }
        ).expect("Should create mcc mapping");

        let card_1 = Wallet::insert_card(
            NewCard {
                user_id: user.id,
                payment_method_id: payment_method_id_1.to_string(),
                credit_card_id: 1,
                wallet_card_attempt_id: att1.id
            }
        ).expect("Should insert card");
        let card_2 = Wallet::insert_card(
            NewCard {
                user_id: user.id,
                payment_method_id: payment_method_id_2.to_string(),
                credit_card_id: 2,
                wallet_card_attempt_id: att2.id
            }
        ).expect("Should insert card");
        let should_be_filtered_rule = Rule::create(
            CreateRuleRequest {
                credit_card_id: 1,
                rule_mcc: Some(rule_mcc.to_string()),
                merchant_name: None,
                points_multiplier: Some(1000),
                cashback_percentage_bips: None,
                recurring_day_of_month: Some(DayOfMonth::FIRST.as_str()),
                start_date: Some(Utc::now().naive_utc().date()),
                end_date: None,
            }
        ).expect("rule should be created");

        let card_1_rule = Rule::create(
            CreateRuleRequest {
                credit_card_id: 1,
                rule_mcc: Some(rule_mcc.to_string()),
                merchant_name: None,
                points_multiplier: Some(2),
                cashback_percentage_bips: None,
                recurring_day_of_month: None,
                start_date: None,
                end_date: None,
            }
        ).expect("rule should be created");

        let card_2_rule = Rule::create(
            CreateRuleRequest {
                credit_card_id: 2,
                rule_mcc: Some(rule_mcc.to_string()),
                merchant_name: None,
                points_multiplier: Some(5),
                cashback_percentage_bips: None,
                recurring_day_of_month: None,
                start_date: None,
                end_date: None,
            }
        ).expect("rule should be created");

        let cards = RuleEngine::order_user_cards_for_request(
            create_example_asa(30000, "0000".to_string()),
            &user
        ).expect("should get cards");
        assert_eq!(cards.len(), 2);
        assert_eq!(cards[0].credit_card_id, 2);
        assert_eq!(cards[0].id, card_2.id);
        assert_eq!(cards[1].credit_card_id, 1);
        assert_eq!(cards[1].id, card_1.id);
        card_2_rule.delete_self().expect("should delete");
        card_1_rule.delete_self().expect("should delete");
        should_be_filtered_rule.delete_self().expect("should delete");
        card_1.delete_self().expect("should delete");
        card_2.delete_self().expect("should delete");
        att1.delete_self().expect("should delete");
        att2.delete_self().expect("should delete");
        mcc_mapping.delete_self().expect("should delete");
        category.delete_self().expect("should delete");
        user.delete_self().expect("should delete");
    }
}