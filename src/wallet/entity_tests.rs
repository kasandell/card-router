#[cfg(test)]
mod entity_tests {
    use crate::wallet::entity::{Wallet, NewCard, WalletCardAttempt, InsertableCardAttempt};
    use crate::test_helper::user::create_user;

    #[actix_web::test]
    async fn test_card_create() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let stripe_pmt_id = "s_1234";

        let attempt = WalletCardAttempt::insert(
            &InsertableCardAttempt {
                user_id: user.id,
                credit_card_id: 1,
                expected_reference_id: ""
            }
        ).await;
        let attempt = attempt.expect("attempt should exist");

        let card = Wallet::insert_card(
            &NewCard {
                user_id: user.id,
                payment_method_id: stripe_pmt_id,
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