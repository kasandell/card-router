#[cfg(test)]
mod tests {
    use crate::credit_card_type::entity::{CreditCardType, CreditCardIssuer, CreditCard};
    use crate::test_helper::initialize_user;
    use crate::rule_engine::engine::{
        RuleEngine,
        WalletDetail
    };
    use crate::wallet::entity::Wallet;
    use crate::rule_engine::entity::Rule;

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
}