#[cfg(test)]
mod entity_tests {
    use crate::wallet::entity::{Wallet, NewCard};
    use crate::test_helper::initialize_user;

    #[actix_web::test]
    async fn test_dupe_create() {
        crate::test::init();
        let user = initialize_user();
        let stripe_pmt_id = "s_1234";

        let card = Wallet::insert_card(
            NewCard {
                user_id: user.id,
                stripe_payment_method_id: stripe_pmt_id.to_string(),
                credit_card_id: 1 // should be populated already
            }
        );
        assert!(card.is_ok());
        let card = card.expect("Card should be ok");
        assert_eq!(stripe_pmt_id, card.stripe_payment_method_id);
        assert_eq!(1, card.credit_card_id);
        assert!(!card.public_id.is_nil());
    }
}