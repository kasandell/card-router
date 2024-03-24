#[cfg(test)]
mod entity_tests {
    use actix_web::test;
    use crate::credit_card_type::constant::CreditCardTypeEnum;
    use crate::error::data_error::DataError;
    use crate::wallet::entity::{Wallet, WalletCardAttempt, InsertableCardAttempt, InsertableCard};
    use crate::test_helper::user::create_user;

    const EXPECTED_REFERENCE_ID: &str = "EXP_REF_123";

    #[test]
    async fn test_card_create() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let stripe_pmt_id = "s_1234";

        let attempt = WalletCardAttempt::insert(
            &InsertableCardAttempt {
                user_id: user.id,
                credit_card_id: CreditCardTypeEnum::ChaseSapphirePreferred.into(),
                expected_reference_id: ""
            }
        ).await;
        let attempt = attempt.expect("attempt should exist");

        let card = Wallet::insert_card(
            &InsertableCard {
                user_id: user.id,
                payment_method_id: stripe_pmt_id,
                credit_card_id: CreditCardTypeEnum::ChaseSapphirePreferred.into(),
                wallet_card_attempt_id: attempt.id
            }
        ).await;
        assert!(card.is_ok());
        let card = card.expect("Card should be ok");
        assert_eq!(stripe_pmt_id, card.payment_method_id);
        assert_eq!(1, card.credit_card_id);
        assert!(!card.public_id.is_nil());
    }


    #[test]
    async fn test_wallet_update_find_by_reference_finds() {

        crate::test_helper::general::init();
        let user = create_user().await;
        let stripe_pmt_id = "s_1234";

        let attempt = WalletCardAttempt::insert(
            &InsertableCardAttempt {
                user_id: user.id,
                credit_card_id: CreditCardTypeEnum::ChaseSapphirePreferred.into(),
                expected_reference_id: EXPECTED_REFERENCE_ID
            }
        ).await.expect("attempt should exist");

        let found = WalletCardAttempt::find_by_reference_id(EXPECTED_REFERENCE_ID).await.expect("should find");
        assert_eq!(found, attempt);
    }

    #[test]
    async fn test_wallet_update_find_by_reference_does_not_find() {
        let stripe_pmt_id = "s_1234";
        let found = WalletCardAttempt::find_by_reference_id(EXPECTED_REFERENCE_ID).await.expect_err("should not find");
        assert_eq!(DataError::NotFound("test".into()), found);
    }

    #[test]
    async fn test_update_card_works() {

    }

    #[test]
    async fn test_update_card_fails() {

    }
}