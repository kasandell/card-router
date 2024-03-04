#[cfg(test)]
mod entity_tests {
    use crate::wallet::entity::{Wallet, NewCard, WalletCardAttempt, InsertableCardAttempt};
    use crate::test_helper::initialize_user;

    #[actix_web::test]
    async fn test_card_create() {
        crate::test::init();
        let user = initialize_user().await;
        let stripe_pmt_id = "s_1234";

        let attempt = WalletCardAttempt::insert(
            InsertableCardAttempt {
                user_id: user.id,
                credit_card_id: 1,
                expected_reference_id: "".to_string()
            }
        ).await;
        let attempt = attempt.expect("attempt should exist");

        let card = Wallet::insert_card(
            NewCard {
                user_id: user.id,
                payment_method_id: stripe_pmt_id.to_string(),
                credit_card_id: 1, // should be populated already,
                wallet_card_attempt_id: attempt.id
            }
        ).await;
        assert!(card.is_ok());
        let card = card.expect("Card should be ok");
        assert_eq!(stripe_pmt_id, card.payment_method_id);
        assert_eq!(1, card.credit_card_id);
        assert!(!card.public_id.is_nil());
        card.delete_self().await.expect("should delete");
        attempt.delete_self().await.expect("should delete");
        user.delete_self().await.expect("should delete");
    }
}