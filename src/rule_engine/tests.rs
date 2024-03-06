#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::category::entity::{Category, InsertableCategory, MccMapping, InsertableMccMapping};
    use crate::credit_card_type::entity::{CreditCardType, CreditCardIssuer, CreditCard};
    use crate::rule_engine::constant::DayOfMonth;
    use crate::rule_engine::request::CreateRuleRequest;
    use crate::test_helper::initialize_user;
    use crate::rule_engine::engine::{
        RuleEngine,
        RuleEngineTrait,
        WalletDetail
    };
    use crate::wallet::entity::{Wallet, NewCard, WalletCardAttempt, InsertableCardAttempt};
    use crate::rule_engine::entity::Rule;
    use crate::asa::request::create_example_asa;
    use chrono::Utc;

    const RULE_CATEGORY: i32 = 1;

    #[actix_web::test]
    async fn test_filter_rules() {
        crate::test::init();
        let amount_cents = 30000;
        let mut cards: Vec<WalletDetail> = Vec::new();
        let rule_engine = Arc::new(RuleEngine::new());
        cards.push(
            (
                Wallet::create_test_wallet(1, 1, 1).await,
                CreditCard::create_test_credit_card(1, "Sapphire".to_string(), 1, 1).await,
                CreditCardType::create_test_credit_card_type(1, "Chase".to_string()),
                CreditCardIssuer::create_test_credit_card_issuer(1, "Visa".to_string())
            )
       );

       cards.push(
            (
                Wallet::create_test_wallet(2, 1, 2).await,
                CreditCard::create_test_credit_card(2, "Bilt".to_string(), 1, 1).await,
                CreditCardType::create_test_credit_card_type(2, "Bilt".to_string()),
                CreditCardIssuer::create_test_credit_card_issuer(1, "MasterCard".to_string())
            )
        );

        let mut rules: Vec<Rule> = Vec::new();
        rules.push(
            Rule::create_test_rule_dateless_mcc_points(1, 1, 2)
        );
        rules.push(
            Rule::create_test_rule_dateless_mcc_points(2, 2, 3)
        );

        let wallet_returned = rule_engine.get_card_order_from_rules(&cards, &rules, amount_cents).await.expect("wallet should come back");
        assert_eq!(wallet_returned[0].credit_card_id, 2);
        assert_eq!(wallet_returned[1].credit_card_id, 1);
        assert_eq!(wallet_returned.len(), 2);
    }


    #[actix_web::test]
    async fn test_filter_rules_cashback_and_points() {
        crate::test::init();
        let amount_cents = 30000;
        let mut cards: Vec<WalletDetail> = Vec::new();
        cards.push(
            (
                Wallet::create_test_wallet(1, 1, 1).await,
                CreditCard::create_test_credit_card(1, "Sapphire".to_string(), 1, 1).await,
                CreditCardType::create_test_credit_card_type(1, "Chase".to_string()),
                CreditCardIssuer::create_test_credit_card_issuer(1, "Visa".to_string())
            )
       );

       cards.push(
            (
                Wallet::create_test_wallet(2, 1, 2).await,
                CreditCard::create_test_credit_card(2, "Bilt".to_string(), 1, 1).await,
                CreditCardType::create_test_credit_card_type(2, "Bilt".to_string()),
                CreditCardIssuer::create_test_credit_card_issuer(1, "MasterCard".to_string())
            )
        );

        let mut rules: Vec<Rule> = Vec::new();
        rules.push(
            Rule::create_test_rule_dateless_mcc_points(1, 1, 2) // 2x point multiple
        );
        rules.push(
            Rule::create_test_rule_dateless_mcc_cashback(2, 2, 250) // 2.5% cashback
        );

        let rule_engine = Arc::new(RuleEngine::new());

        let wallet_returned = rule_engine.get_card_order_from_rules(&cards, &rules, amount_cents).await.expect("wallet should come back");
        assert_eq!(wallet_returned[0].credit_card_id, 2);
        assert_eq!(wallet_returned[1].credit_card_id, 1);
        assert_eq!(wallet_returned.len(), 2);
    }

    //#[actix_web::test]
    // TODO: disabled while we can't get category to insert properly
    async fn test_order_user_cards_for_request() {
        crate::test::init();
        let user = initialize_user().await;
        let payment_method_id_1 = "s_1234";
        let payment_method_id_2 = "s_1235";
        let rule_mcc = "0000";

        let att1 = WalletCardAttempt::insert(
            &InsertableCardAttempt {
                user_id: user.id,
                credit_card_id: 1,
                expected_reference_id: "a"
            }
        ).await.expect("expect attempt to create");

        let att2 = WalletCardAttempt::insert(
            &InsertableCardAttempt {
                user_id: user.id,
                credit_card_id: 2,
                expected_reference_id: "b"
            }
        ).await.expect("expect attempt to create");

        let category = Category::create(
            &InsertableCategory {
                name: "Test"
            }
        ).await.expect("should create category");
        let mcc_mapping = MccMapping::create(
            &InsertableMccMapping {
                mcc_code: rule_mcc,
                category_id: 1,//category.id
            }
        ).await.expect("Should create mcc mapping");

        let card_1 = Wallet::insert_card(
            &NewCard {
                user_id: user.id,
                payment_method_id: payment_method_id_1,
                credit_card_id: 1,
                wallet_card_attempt_id: att1.id
            }
        ).await.expect("Should insert card");
        let card_2 = Wallet::insert_card(
            &NewCard {
                user_id: user.id,
                payment_method_id: payment_method_id_2,
                credit_card_id: 2,
                wallet_card_attempt_id: att2.id
            }
        ).await.expect("Should insert card");
        let should_be_filtered_rule = Rule::create(
            &CreateRuleRequest {
                credit_card_id: 1,
                rule_category_id: Some(RULE_CATEGORY),
                merchant_name: None,
                points_multiplier: Some(1000),
                cashback_percentage_bips: None,
                recurring_day_of_month: Some(DayOfMonth::FIRST.as_str()),
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

        let rule_engine = Arc::new(RuleEngine::new());
        let cards = rule_engine.clone().order_user_cards_for_request(
            &create_example_asa(30000, "0000".to_string()),
            &user
        ).await.expect("should get cards");
        assert_eq!(cards.len(), 2);
        assert_eq!(cards[0].credit_card_id, 2);
        assert_eq!(cards[0].id, card_2.id);
        assert_eq!(cards[1].credit_card_id, 1);
        assert_eq!(cards[1].id, card_1.id);
        card_2_rule.delete_self().await.expect("should delete");
        card_1_rule.delete_self().await.expect("should delete");
        should_be_filtered_rule.delete_self().await.expect("should delete");
        card_1.delete_self().await.expect("should delete");
        card_2.delete_self().await.expect("should delete");
        att1.delete_self().await.expect("should delete");
        att2.delete_self().await.expect("should delete");
        mcc_mapping.delete_self().await.expect("should delete");
        //category.delete_self().await.expect("should delete");
        user.delete_self().await.expect("should delete");
    }
}