// TODO: everything needs a rewrite
#[cfg(test)]
mod tests {
    use std::ops::Add;
    use actix_web::test;
    use std::sync::Arc;
    use diesel_async::AsyncConnection;
    use footprint::models::CreateClientTokenResponse;
    use mockall::predicate::eq;
    use mockall::Sequence;
    use uuid::Uuid;
    use crate::footprint::service::MockFootprintServiceTrait;
    use crate::credit_card_type::service::MockCreditCardServiceTrait;
    use crate::error::data_error::DataError;
    use crate::footprint::error::FootprintError;
    use crate::test_helper::{
        credit_card::create_mock_credit_card,
        user::create_mock_user
    };
    use crate::test_helper::user::create_user;
    use crate::test_helper::wallet::create_mock_wallet_attempt;
    use crate::util::db;
    use crate::wallet::constant::WalletCardAttemptStatus;
    use crate::wallet::entity::{UpdateCardAttempt, WalletCardAttempt};
    use crate::wallet::error::WalletError;
    use crate::wallet::service::{WalletService, WalletServiceTrait};
    use crate::wallet::request::{MatchRequest, RegisterAttemptRequest};

    const USER_ID: i32 = 1;
    const CREDIT_CARD_ID: i32 = 1;
    const CREDIT_CARD_PUBLIC_ID: Uuid = Uuid::from_u128(0x9cb4cf49_5c3d_4647_83b0_8f3515da7be1);
    const CREDIT_CARD_NAME: &str = "Sapphire Reserve";

    const TOKEN: &str = "12345";

    #[test]
    async fn test_register_attempt() {
        crate::test_helper::general::init();
        let mut footprint_service = MockFootprintServiceTrait::new();
        let mut credit_card_service = MockCreditCardServiceTrait::new();

        let cc = create_mock_credit_card(CREDIT_CARD_NAME);

        let mut wca = create_mock_wallet_attempt();

        let user = create_user().await;

        footprint_service.expect_create_client_token()
            .times(1)
            .return_once(
                move |_, _|
                Ok(CreateClientTokenResponse {
                    expires_at: None,
                    token: TOKEN.to_string()
                })
            );

        let cloned_card = cc.clone();
        credit_card_service.expect_find_by_public_id()
            .times(1)
            .with(eq(CREDIT_CARD_PUBLIC_ID))
            .return_once(move |_| Ok(cloned_card));

        let wallet_engine = Arc::new(WalletService::new_with_services(
            Arc::new(credit_card_service),
            Arc::new(footprint_service)
        ));

        let wca_ret = wallet_engine.clone().register_new_attempt(
            &user,
            &RegisterAttemptRequest {
                credit_card_type_public_id: CREDIT_CARD_PUBLIC_ID,
            }
        ).await.expect("no error");

        assert_eq!(TOKEN, wca_ret.token.as_str());
    }

    #[test]
    async fn test_register_attempt_fails() {
        crate::test_helper::general::init();
        let mut footprint_service = MockFootprintServiceTrait::new();
        let mut credit_card_service = MockCreditCardServiceTrait::new();

        let cc = create_mock_credit_card(CREDIT_CARD_NAME);
        // note that we create a mock user here, but not actually one in the database
        let user = create_mock_user();

        footprint_service.expect_create_client_token()
            .times(0);


        credit_card_service.expect_find_by_public_id()
            .times(1)
            .with(eq(CREDIT_CARD_PUBLIC_ID))
            .return_once(move |_| Ok(cc.clone()));
        let wallet_engine = Arc::new(WalletService::new_with_services(
            Arc::new(credit_card_service),
            Arc::new(footprint_service)
        ));

        let err: WalletError = wallet_engine.clone().register_new_attempt(
            &user,
            &RegisterAttemptRequest {
                credit_card_type_public_id: CREDIT_CARD_PUBLIC_ID,
            }
        ).await.expect_err("should return error");

        assert_eq!(WalletError::Unexpected("test".into()), err);
    }

    #[test]
    async fn test_register_attempt_fails_create_token() {
        crate::test_helper::general::init();
        let mut footprint_service = MockFootprintServiceTrait::new();
        let mut credit_card_service = MockCreditCardServiceTrait::new();

        let cc = create_mock_credit_card(CREDIT_CARD_NAME);
        let user = create_user().await;

        footprint_service.expect_create_client_token()
            .times(1)
            .return_once(move |_, _| Err(FootprintError::Unexpected("test".into())));

        credit_card_service.expect_find_by_public_id()
            .times(1)
            .with(eq(CREDIT_CARD_PUBLIC_ID))
            .return_once(move |_| Ok(cc.clone()));

        let wallet_engine = Arc::new(WalletService::new_with_services(
            Arc::new(credit_card_service),
            Arc::new(footprint_service)
        ));

        let err: WalletError = wallet_engine.clone().register_new_attempt(
            &user,
            &RegisterAttemptRequest {
                credit_card_type_public_id: CREDIT_CARD_PUBLIC_ID,
            }
        ).await.expect_err("should return error");

        assert_eq!(WalletError::Unexpected("test".into()), err);
    }

    #[test]
    async fn test_register_attempt_several() {
        crate::test_helper::general::init();
        let mut footprint_service = MockFootprintServiceTrait::new();
        let mut credit_card_service = MockCreditCardServiceTrait::new();

        let cc = create_mock_credit_card(CREDIT_CARD_NAME);
        let cc_cloned = cc.clone();

        let user = create_user().await;

        let mut sequence = Sequence::new();
        footprint_service.expect_create_client_token()
            .once()
            .in_sequence(&mut sequence)
            .return_once(move |_, _|
                Ok(CreateClientTokenResponse {
                    expires_at: None,
                    token: TOKEN.to_string(),
                })
            );

        let token_2 = TOKEN.to_string().add("_test".into());
        let token_2_clone = token_2.clone();
        footprint_service.expect_create_client_token()
            .once()
            .in_sequence(&mut sequence)
            .return_once(move |_, _|
                Ok(CreateClientTokenResponse {
                    expires_at: None,
                    token: token_2_clone
                })
            );

        let mut seq_2 = Sequence::new();

        credit_card_service.expect_find_by_public_id()
            .once()
            .in_sequence(&mut seq_2)
            .with(eq(CREDIT_CARD_PUBLIC_ID))
            .return_once(move |_|
                Ok(cc)
            );

        credit_card_service.expect_find_by_public_id()
            .once()
            .in_sequence(&mut seq_2)
            .with(eq(CREDIT_CARD_PUBLIC_ID))
            .return_once(move |_|
                Ok(cc_cloned)
            );

        let wallet_engine = Arc::new(WalletService::new_with_services(
            Arc::new(credit_card_service),
            Arc::new(footprint_service)
        ));

        let wca_ret = wallet_engine.clone().register_new_attempt(
            &user,
            &RegisterAttemptRequest {
                credit_card_type_public_id: CREDIT_CARD_PUBLIC_ID,
            }
        ).await.expect("no error");

        assert_eq!(wca_ret.token, TOKEN.to_string());

        let wca_ret2 = wallet_engine.clone().register_new_attempt(
            &user,
            &RegisterAttemptRequest {
                credit_card_type_public_id: CREDIT_CARD_PUBLIC_ID,
            }
        ).await.expect("no error");
        assert_eq!(wca_ret2.token, token_2);

        assert_ne!(wca_ret2.reference_id, wca_ret.reference_id);
        assert_ne!(wca_ret2.token, wca_ret.token);
    }

    #[test]
    async fn test_match_find() {
        crate::test_helper::general::init();
        let mut credit_card_service = MockCreditCardServiceTrait::new();
        let mut footprint_service = MockFootprintServiceTrait::new();

        let cc = create_mock_credit_card(CREDIT_CARD_NAME);
        let cc_cloned = cc.clone();
        credit_card_service.expect_find_by_public_id()
            .times(1)
            .with(eq(CREDIT_CARD_PUBLIC_ID))
            .return_once(
                move |_| Ok(cc_cloned)
            );

        footprint_service.expect_create_client_token()
            .once()
            .return_once(move |_, _|
                Ok(CreateClientTokenResponse {
                    expires_at: None,
                    token: TOKEN.to_string(),
                })
            );

        let user = create_user().await;

        let wallet_engine = Arc::new(WalletService::new_with_services(
            Arc::new(credit_card_service),
            Arc::new(footprint_service)
        ));

        let attempt = wallet_engine.clone().register_new_attempt(
            &user,
            &RegisterAttemptRequest {
                credit_card_type_public_id: CREDIT_CARD_PUBLIC_ID,
            }
        ).await.expect("creates ok");

        let created_card = wallet_engine.clone().match_card(
            &user,
            &MatchRequest {
                reference_id: attempt.reference_id.clone()
            }
        ).await.expect("should be ok");

        assert_eq!(created_card.credit_card_id, cc.id);
        assert_eq!(created_card.user_id, user.id);
        let attempt_in_db = wallet_engine.wallet_card_attempt_dao.clone().find_by_reference_id(
            attempt.reference_id.as_str()
        ).await.expect("Should find in db");
        assert_eq!(created_card.payment_method_id, attempt_in_db.expected_reference_id.to_string());
        assert_eq!(created_card.wallet_card_attempt_id, attempt_in_db.id);
        assert_eq!(attempt_in_db.status, WalletCardAttemptStatus::Matched);
    }

    #[test]
    async fn test_match_fails_already_matched() {

        crate::test_helper::general::init();
        let mut credit_card_service = MockCreditCardServiceTrait::new();
        let mut footprint_service = MockFootprintServiceTrait::new();

        let cc = create_mock_credit_card(CREDIT_CARD_NAME);
        let cc_cloned = cc.clone();
        credit_card_service.expect_find_by_public_id()
            .times(1)
            .with(eq(CREDIT_CARD_PUBLIC_ID))
            .return_once(
                move |_| Ok(cc_cloned)
            );

        footprint_service.expect_create_client_token()
            .once()
            .return_once(move |_, _|
                Ok(CreateClientTokenResponse {
                    expires_at: None,
                    token: TOKEN.to_string(),
                })
            );

        let user = create_user().await;

        let wallet_engine = Arc::new(WalletService::new_with_services(
            Arc::new(credit_card_service),
            Arc::new(footprint_service)
        ));

        let attempt = wallet_engine.clone().register_new_attempt(
            &user,
            &RegisterAttemptRequest {
                credit_card_type_public_id: CREDIT_CARD_PUBLIC_ID,
            }
        ).await.expect("creates ok");

        let created_card = wallet_engine.clone().match_card(
            &user,
            &MatchRequest {
                reference_id: attempt.reference_id.clone()
            }
        ).await.expect("should be ok");

        assert_eq!(created_card.credit_card_id, cc.id);
        assert_eq!(created_card.user_id, user.id);
        let mut attempt_in_db = wallet_engine.wallet_card_attempt_dao.clone().find_by_reference_id(
            attempt.reference_id.as_str()
        ).await.expect("Should find in db");
        assert_eq!(created_card.payment_method_id, attempt_in_db.expected_reference_id.to_string());
        assert_eq!(created_card.wallet_card_attempt_id, attempt_in_db.id);
        assert_eq!(attempt_in_db.status, WalletCardAttemptStatus::Matched);
        let mut cards_in_db = wallet_engine.wallet_dao.clone().find_all_for_user(
            &user
        ).await.expect("should get");
        assert_eq!(1, cards_in_db.len());
        let mut card = cards_in_db.get(0).unwrap();
        assert_eq!(card.credit_card_id, attempt_in_db.credit_card_id);
        assert_eq!(card.wallet_card_attempt_id, attempt_in_db.id);
        assert_eq!(card.user_id, user.id);
        assert_eq!(card.payment_method_id, attempt_in_db.expected_reference_id);
        let created_at = card.created_at;
        let updated_at = card.updated_at;
        let attempt_created_at = attempt_in_db.created_at;
        let attempt_updated_at = attempt_in_db.updated_at;

        let error = wallet_engine.clone().match_card(
            &user,
            &MatchRequest {
                reference_id: attempt.reference_id.clone(),
            }
        ).await.expect_err("Should fail to match twice");
        assert_eq!(WalletError::Conflict("test".into()), error);
        attempt_in_db = wallet_engine.wallet_card_attempt_dao.clone().find_by_reference_id(
            attempt.reference_id.as_str()
        ).await.expect("Should find in db");
        assert_eq!(created_card.payment_method_id, attempt_in_db.expected_reference_id.to_string());
        assert_eq!(created_card.wallet_card_attempt_id, attempt_in_db.id);
        assert_eq!(attempt_in_db.status, WalletCardAttemptStatus::Matched);
        cards_in_db = wallet_engine.wallet_dao.clone().find_all_for_user(
            &user
        ).await.expect("should get");
        assert_eq!(1, cards_in_db.len());
        card = cards_in_db.get(0).unwrap();
        assert_eq!(card.credit_card_id, attempt_in_db.credit_card_id);
        assert_eq!(card.wallet_card_attempt_id, attempt_in_db.id);
        assert_eq!(card.user_id, user.id);
        assert_eq!(card.payment_method_id, attempt_in_db.expected_reference_id);
        assert_eq!(attempt_in_db.created_at, attempt_created_at);
        assert_eq!(attempt_in_db.updated_at, attempt_updated_at);
        assert_eq!(card.created_at, created_at);
        assert_eq!(card.updated_at, updated_at);
    }

    #[test]
    async fn test_match_fails_unauthorized() {
        crate::test_helper::general::init();
        let mut credit_card_service = MockCreditCardServiceTrait::new();
        let mut footprint_service = MockFootprintServiceTrait::new();

        let cc = create_mock_credit_card(CREDIT_CARD_NAME);
        let cc_cloned = cc.clone();
        credit_card_service.expect_find_by_public_id()
            .times(1)
            .with(eq(CREDIT_CARD_PUBLIC_ID))
            .return_once(
                move |_| Ok(cc_cloned)
            );

        footprint_service.expect_create_client_token()
            .once()
            .return_once(move |_, _|
                Ok(CreateClientTokenResponse {
                    expires_at: None,
                    token: TOKEN.to_string(),
                })
            );

        let user = create_user().await;
        let user2 = create_user().await;

        let wallet_engine = Arc::new(WalletService::new_with_services(
            Arc::new(credit_card_service),
            Arc::new(footprint_service)
        ));

        let attempt = wallet_engine.clone().register_new_attempt(
            &user,
            &RegisterAttemptRequest {
                credit_card_type_public_id: CREDIT_CARD_PUBLIC_ID,
            }
        ).await.expect("creates ok");

        let wallet_error = wallet_engine.clone().match_card(
            &user2,
            &MatchRequest {
                reference_id: attempt.reference_id.clone()
            }
        ).await.expect_err("should not match");
        assert_eq!(WalletError::Unauthorized("test".into()), wallet_error);
        let attempt_in_db = wallet_engine.wallet_card_attempt_dao.clone().find_by_reference_id(
            attempt.reference_id.as_str()
        ).await.expect("Should find in db");
        assert_eq!(attempt_in_db.status, WalletCardAttemptStatus::Pending);
        assert_eq!(attempt_in_db.user_id, user.id);
        let mut cards_in_db = wallet_engine.wallet_dao.clone().find_all_for_user(&user).await.expect("should get cards");
        assert_eq!(0, cards_in_db.len());
        cards_in_db = wallet_engine.wallet_dao.clone().find_all_for_user(&user2).await.expect("should get cards");
        assert_eq!(0, cards_in_db.len());
    }

    #[test]
    async fn test_match_fails_to_find() {
        crate::test_helper::general::init();
        let mut credit_card_service = MockCreditCardServiceTrait::new();
        let mut footprint_service = MockFootprintServiceTrait::new();

        let user = create_user().await;

        let wallet_engine = Arc::new(WalletService::new_with_services(
            Arc::new(credit_card_service),
            Arc::new(footprint_service)
        ));

        let attempt_id = "test";
        let wallet_error = wallet_engine.clone().match_card(
            &user,
            &MatchRequest {
                reference_id: attempt_id.clone().into(),
            }
        ).await.expect_err("should not match");
        assert_eq!(WalletError::NotFound("test".into()), wallet_error);
        let attempt_error = wallet_engine.wallet_card_attempt_dao.clone().find_by_reference_id(
            attempt_id
        ).await.expect_err("Should not find in db");
        assert_eq!(DataError::NotFound("test".into()), attempt_error);
        let mut cards_in_db = wallet_engine.wallet_dao.clone().find_all_for_user(&user).await.expect("should get cards");
        assert_eq!(0, cards_in_db.len());
    }

    #[test]
    async fn test_cannot_match_when_wca_already_attached_to_card() {
        crate::test_helper::general::init();
        let mut credit_card_service = MockCreditCardServiceTrait::new();
        let mut footprint_service = MockFootprintServiceTrait::new();

        let cc = create_mock_credit_card(CREDIT_CARD_NAME);
        let cc_cloned = cc.clone();
        credit_card_service.expect_find_by_public_id()
            .times(1)
            .with(eq(CREDIT_CARD_PUBLIC_ID))
            .return_once(
                move |_| Ok(cc_cloned)
            );

        footprint_service.expect_create_client_token()
            .once()
            .return_once(move |_, _|
                Ok(CreateClientTokenResponse {
                    expires_at: None,
                    token: TOKEN.to_string(),
                })
            );

        let user = create_user().await;

        let wallet_engine = Arc::new(WalletService::new_with_services(
            Arc::new(credit_card_service),
            Arc::new(footprint_service)
        ));

        let attempt = wallet_engine.clone().register_new_attempt(
            &user,
            &RegisterAttemptRequest {
                credit_card_type_public_id: CREDIT_CARD_PUBLIC_ID,
            }
        ).await.expect("creates ok");

        let created_card = wallet_engine.clone().match_card(
            &user,
            &MatchRequest {
                reference_id: attempt.reference_id.clone()
            }
        ).await.expect("should be ok");

        assert_eq!(created_card.credit_card_id, cc.id);
        assert_eq!(created_card.user_id, user.id);
        let mut attempt_in_db = wallet_engine.wallet_card_attempt_dao.clone().find_by_reference_id(
            attempt.reference_id.as_str()
        ).await.expect("Should find in db");
        assert_eq!(created_card.payment_method_id, attempt_in_db.expected_reference_id.to_string());
        assert_eq!(created_card.wallet_card_attempt_id, attempt_in_db.id);
        assert_eq!(attempt_in_db.status, WalletCardAttemptStatus::Matched);
        let mut cards_in_db = wallet_engine.wallet_dao.clone().find_all_for_user(
            &user
        ).await.expect("should get");
        assert_eq!(1, cards_in_db.len());
        let mut card = cards_in_db.get(0).unwrap();
        assert_eq!(card.credit_card_id, attempt_in_db.credit_card_id);
        assert_eq!(card.wallet_card_attempt_id, attempt_in_db.id);
        assert_eq!(card.user_id, user.id);
        assert_eq!(card.payment_method_id, attempt_in_db.expected_reference_id);
        let created_at = card.created_at;
        let updated_at = card.updated_at;
        
        let attempt_in_db_pending= wallet_engine.wallet_card_attempt_dao.clone().update_card(
            attempt_in_db.id,
            &UpdateCardAttempt {
                status: WalletCardAttemptStatus::Pending,
            }
        ).await.expect("should update");
        assert_eq!(attempt_in_db_pending.status, WalletCardAttemptStatus::Pending);

        let error = wallet_engine.clone().match_card(
            &user,
            &MatchRequest {
                reference_id: attempt.reference_id.clone(),
            }
        ).await.expect_err("Should fail to match twice");
        assert_eq!(WalletError::Conflict("test".into()), error);
        attempt_in_db= wallet_engine.wallet_card_attempt_dao.clone().find_by_reference_id(
            attempt_in_db_pending.expected_reference_id.as_str()
        ).await.expect("Should find in db");
        assert_eq!(created_card.payment_method_id, attempt_in_db.expected_reference_id.to_string());
        assert_eq!(created_card.wallet_card_attempt_id, attempt_in_db.id);
        assert_eq!(attempt_in_db.status, WalletCardAttemptStatus::Pending);
        cards_in_db = wallet_engine.wallet_dao.clone().find_all_for_user(
            &user
        ).await.expect("should get");
        assert_eq!(1, cards_in_db.len());
        card = cards_in_db.get(0).unwrap();
        assert_eq!(card.credit_card_id, attempt_in_db.credit_card_id);
        assert_eq!(card.wallet_card_attempt_id, attempt_in_db.id);
        assert_eq!(card.user_id, user.id);
        assert_eq!(card.payment_method_id, attempt_in_db.expected_reference_id);
        assert_eq!(card.created_at, created_at);
        assert_eq!(card.updated_at, updated_at);
    }
}