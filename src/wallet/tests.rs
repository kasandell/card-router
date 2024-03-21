// TODO: everything needs a rewrite
#[cfg(test)]
mod tests {
    use actix_web;
    use std::sync::Arc;
    use diesel::serialize::ToSql;
    use footprint::models::CreateClientTokenResponse;
    use mockall::predicate::eq;
    use uuid::Uuid;
    use crate::adyen::checkout::service::MockAdyenChargeServiceTrait;
    use crate::error::api_error::ApiError;
    use crate::credit_card_type::dao::MockCreditCardDaoTrait;
    use crate::error::data_error::DataError;
    use crate::error::service_error::ServiceError;
    use crate::footprint::service::MockFootprintServiceTrait;
    use crate::test_helper::{
        credit_card::create_mock_credit_card,
        wallet::create_mock_wallet,
        user::create_mock_user
    };
    use crate::test_helper::wallet::create_mock_wallet_attempt;

    use crate::wallet::constant::WalletCardAttemptStatus;
    use crate::wallet::dao::{MockWalletCardAttemtDaoTrait, MockWalletDaoTrait};
    use crate::wallet::service::WalletService;
    use crate::wallet::request::{MatchAttemptRequest, MatchRequest, RegisterAttemptRequest};

    const USER_ID: i32 = 1;
    const CREDIT_CARD_ID: i32 = 1;
    const CREDIT_CARD_PUBLIC_ID: Uuid = Uuid::from_u128(0x9cb4cf49_5c3d_4647_83b0_8f3515da7be1);
    const CREDIT_CARD_NAME: &str = "Sapphire Reserve";

    const TOKEN: &str = "12345";

    #[actix_web::test]
    async fn test_register_attempt() {
        let mut cc_dao = MockCreditCardDaoTrait::new();
        let mut wca_dao = MockWalletCardAttemtDaoTrait::new();
        let w_dao = MockWalletDaoTrait::new();
        let adyen_service = MockAdyenChargeServiceTrait::new();
        let mut footprint_service = MockFootprintServiceTrait::new();

        let cc = create_mock_credit_card(CREDIT_CARD_NAME);

        let mut wca = create_mock_wallet_attempt();
        let expected_reference_id = Uuid::new_v4().to_string();
        wca.expected_reference_id = expected_reference_id.clone();

        let user = create_mock_user();

        footprint_service.expect_create_client_token()
            .times(1)
            .return_const(
                Ok(CreateClientTokenResponse {
                    expires_at: None,
                    token: TOKEN.to_string()
                })
            );

        wca_dao.expect_insert()
            .times(1)
            .withf(move |insert_request| {
                insert_request.user_id == USER_ID
                && insert_request.credit_card_id == CREDIT_CARD_ID
            })
            .return_const(
                Ok(wca.clone())
            );

        cc_dao.expect_find_by_public_id()
            .times(1)
            .with(eq(CREDIT_CARD_PUBLIC_ID))
            .return_const(
                Ok(cc.clone())
            );

        let wallet_engine = Arc::new(WalletService::new_with_services(
            Arc::new(cc_dao),
            Arc::new(wca_dao),
            Arc::new(w_dao),
            Arc::new(adyen_service),
            Arc::new(footprint_service)
        ));

        let wca_ret = wallet_engine.clone().register_new_attempt(
            &user,
            &RegisterAttemptRequest {
                credit_card_type_public_id: CREDIT_CARD_PUBLIC_ID,
            }
        ).await.expect("no error");

        assert_eq!(wca.expected_reference_id, wca_ret.reference_id);
        assert_eq!(TOKEN, wca_ret.token.as_str());
    }

    #[actix_web::test]
    async fn test_register_attempt_fails() {
        let mut cc_dao = MockCreditCardDaoTrait::new();
        let mut wca_dao = MockWalletCardAttemtDaoTrait::new();
        let mut w_dao = MockWalletDaoTrait::new();
        let mut adyen_service = MockAdyenChargeServiceTrait::new();
        let mut footprint_service = MockFootprintServiceTrait::new();

        let cc = create_mock_credit_card(CREDIT_CARD_NAME);
        let wca = create_mock_wallet_attempt();
        let user = create_mock_user();

        footprint_service.expect_create_client_token()
            .times(0);

        wca_dao.expect_insert()
            .times(1)
            .withf(move |insert_request| {
                insert_request.user_id == USER_ID
                    && insert_request.credit_card_id == CREDIT_CARD_ID
            })
            .return_const(
                Err(DataError::new(ErrorType::InternalServerError, "test error"))
            );

        cc_dao.expect_find_by_public_id()
            .times(1)
            .with(eq(CREDIT_CARD_PUBLIC_ID))
            .return_const(
                Ok(cc.clone())
            );

        let wallet_engine = Arc::new(WalletService::new_with_services(
            Arc::new(cc_dao),
            Arc::new(wca_dao),
            Arc::new(w_dao),
            Arc::new(adyen_service),
            Arc::new(footprint_service)
        ));

        let err: ApiError = wallet_engine.clone().register_new_attempt(
            &user,
            &RegisterAttemptRequest {
                credit_card_type_public_id: CREDIT_CARD_PUBLIC_ID,
            }
        ).await.expect_err("should return error");

        assert_eq!(ErrorType::InternalServerError, err.error_type);
    }

    #[actix_web::test]
    async fn test_register_attempt_fails_create_token() {
        let mut cc_dao = MockCreditCardDaoTrait::new();
        let mut wca_dao = MockWalletCardAttemtDaoTrait::new();
        let mut w_dao = MockWalletDaoTrait::new();
        let mut adyen_service = MockAdyenChargeServiceTrait::new();
        let mut footprint_service = MockFootprintServiceTrait::new();

        let cc = create_mock_credit_card(CREDIT_CARD_NAME);
        let wca = create_mock_wallet_attempt();
        let user = create_mock_user();

        footprint_service.expect_create_client_token()
            .times(1)
            .return_const(
                Err(ServiceError::new(ErrorType::Unauthorized, "Something wrong"))
            );

        wca_dao.expect_insert()
            .times(1)
            .withf(move |insert_request| {
                insert_request.user_id == USER_ID
                    && insert_request.credit_card_id == CREDIT_CARD_ID
            })
            .return_const(
                Ok(wca.clone())
            );

        cc_dao.expect_find_by_public_id()
            .times(1)
            .with(eq(CREDIT_CARD_PUBLIC_ID))
            .return_const(
                Ok(cc.clone())
            );

        let wallet_engine = Arc::new(WalletService::new_with_services(
            Arc::new(cc_dao),
            Arc::new(wca_dao),
            Arc::new(w_dao),
            Arc::new(adyen_service),
            Arc::new(footprint_service)
        ));

        let err: ApiError = wallet_engine.clone().register_new_attempt(
            &user,
            &RegisterAttemptRequest {
                credit_card_type_public_id: CREDIT_CARD_PUBLIC_ID,
            }
        ).await.expect_err("should return error");

        assert_eq!(ErrorType::Unauthorized, err.error_type);
    }

    #[actix_web::test]
    async fn test_register_attempt_several() {
        let mut cc_dao = MockCreditCardDaoTrait::new();
        let mut wca_dao = MockWalletCardAttemtDaoTrait::new();
        let mut w_dao = MockWalletDaoTrait::new();
        let mut adyen_service = MockAdyenChargeServiceTrait::new();
        let mut footprint_service = MockFootprintServiceTrait::new();

        let cc = create_mock_credit_card(CREDIT_CARD_NAME);
        let wca = create_mock_wallet_attempt();

        let user = create_mock_user();

        let expected_reference_id = Uuid::new_v4().to_string();
        let expected_reference_id_clone = expected_reference_id.clone();

        footprint_service.expect_create_client_token()
            .times(2)
            .return_const(
                Ok(CreateClientTokenResponse {
                    expires_at: None,
                    token: TOKEN.to_string(),
                })
            );
        wca_dao.expect_insert()
            .times(2)
            .withf(move |insert_request| {
                insert_request.user_id == USER_ID
                    && insert_request.credit_card_id == CREDIT_CARD_ID
            })
            .return_const(
                Ok(wca.clone())
            );

        cc_dao.expect_find_by_public_id()
            .times(2)
            .with(eq(CREDIT_CARD_PUBLIC_ID))
            .return_const(
                Ok(cc.clone())
            );

        let wallet_engine = Arc::new(WalletService::new_with_services(
            Arc::new(cc_dao),
            Arc::new(wca_dao),
            Arc::new(w_dao),
            Arc::new(adyen_service),
            Arc::new(footprint_service)
        ));

        let wca_ret = wallet_engine.clone().register_new_attempt(
            &user,
            &RegisterAttemptRequest {
                credit_card_type_public_id: CREDIT_CARD_PUBLIC_ID,
            }
        ).await.expect("no error");

        assert_eq!(wca.expected_reference_id, wca_ret.reference_id);

        let wca_ret2 = wallet_engine.clone().register_new_attempt(
            &user,
            &RegisterAttemptRequest {
                credit_card_type_public_id: CREDIT_CARD_PUBLIC_ID,
            }
        ).await.expect("no error");
        assert_eq!(wca.expected_reference_id, wca_ret2.reference_id);

    }

    #[actix_web::test]
    async fn test_match_find() {
        let cc_dao = MockCreditCardDaoTrait::new();
        let mut wca_dao = MockWalletCardAttemtDaoTrait::new();
        let mut w_dao = MockWalletDaoTrait::new();
        let adyen_service = MockAdyenChargeServiceTrait::new();
        let footprint_service = MockFootprintServiceTrait::new();

        let cc = create_mock_credit_card(CREDIT_CARD_NAME);
        let wca = create_mock_wallet_attempt();


        let user = create_mock_user();

        let expected_reference_id = Uuid::new_v4().to_string();
        let expected_reference_id_clone = expected_reference_id.clone();
        let expected_reference_id_clone_2 = expected_reference_id.clone();

        let wallet_card = create_mock_wallet();

        let mut matched = wca.clone();
        matched.id = 1;
        matched.credit_card_id = 1;
        matched.expected_reference_id = expected_reference_id.clone();
        matched.status = WalletCardAttemptStatus::Matched;

        wca_dao.expect_find_by_reference_id()
            .times(1)
            .with(eq(expected_reference_id.clone()))
            .return_const(
                Ok(wca.clone())
            );

        wca_dao.expect_update_card()
            .times(1)
            .withf(move |card_id, card_attempt| {
                *card_id == wca.id
                &&  card_attempt.status == WalletCardAttemptStatus::Matched

            })
            .return_const(
                Ok(matched.clone())
            );

        w_dao.expect_insert_card()
            .times(1)
            .withf(move |new_card| {
                new_card.user_id == USER_ID
                && new_card.payment_method_id == expected_reference_id_clone_2
                && new_card.credit_card_id == 1
                && new_card.wallet_card_attempt_id == 1
            })
            .return_const(
                Ok(wallet_card.clone())
            );

        let wallet_engine = Arc::new(WalletService::new_with_services(
            Arc::new(cc_dao),
            Arc::new(wca_dao),
            Arc::new(w_dao),
            Arc::new(adyen_service),
            Arc::new(footprint_service)
        ));

        let created_card = wallet_engine.clone().match_card(
            &user,
            &MatchRequest {
                reference_id: expected_reference_id.clone()
            }
        ).await.expect("should be ok");

        assert_eq!(created_card, wallet_card);
    }

    #[actix_web::test]
    async fn test_match_fails_already_matched() {
        let cc_dao = MockCreditCardDaoTrait::new();
        let mut wca_dao = MockWalletCardAttemtDaoTrait::new();
        let mut w_dao = MockWalletDaoTrait::new();
        let adyen_service = MockAdyenChargeServiceTrait::new();
        let footprint_service = MockFootprintServiceTrait::new();

        let cc = create_mock_credit_card(CREDIT_CARD_NAME);
        let user = create_mock_user();
        let expected_reference_id = Uuid::new_v4().to_string();

        let mut matched = create_mock_wallet_attempt();
        matched.id = 1;
        matched.credit_card_id = 1;
        matched.expected_reference_id = expected_reference_id.clone();
        matched.status = WalletCardAttemptStatus::Matched;

        wca_dao.expect_find_by_reference_id()
            .times(1)
            .with(eq(expected_reference_id.clone()))
            .return_const(
                Ok(matched.clone())
            );

        wca_dao.expect_update_card()
            .times(0);


        w_dao.expect_insert_card()
            .times(0);


        let wallet_engine = Arc::new(WalletService::new_with_services(
            Arc::new(cc_dao),
            Arc::new(wca_dao),
            Arc::new(w_dao),
            Arc::new(adyen_service),
            Arc::new(footprint_service)
        ));

        let error = wallet_engine.clone().match_card(
            &user,
            &MatchRequest {
                reference_id: expected_reference_id.clone(),
            }
        ).await.expect_err("should throw error on match");

        assert_eq!(ErrorType::Conflict, error.error_type);
    }

    #[actix_web::test]
    async fn test_match_fails_unauthorized() {
        let cc_dao = MockCreditCardDaoTrait::new();
        let mut wca_dao = MockWalletCardAttemtDaoTrait::new();
        let mut w_dao = MockWalletDaoTrait::new();
        let adyen_service = MockAdyenChargeServiceTrait::new();
        let footprint_service = MockFootprintServiceTrait::new();

        let cc = create_mock_credit_card(CREDIT_CARD_NAME);
        let user = create_mock_user();
        let expected_reference_id = Uuid::new_v4().to_string();

        let mut matched = create_mock_wallet_attempt();
        matched.id = 1;
        matched.credit_card_id = 1;
        matched.expected_reference_id = expected_reference_id.clone();
        matched.user_id = 2;

        wca_dao.expect_find_by_reference_id()
            .times(1)
            .with(eq(expected_reference_id.clone()))
            .return_const(
                Ok(matched.clone())
            );

        wca_dao.expect_update_card()
            .times(0);


        w_dao.expect_insert_card()
            .times(0);


        let wallet_engine = Arc::new(WalletService::new_with_services(
            Arc::new(cc_dao),
            Arc::new(wca_dao),
            Arc::new(w_dao),
            Arc::new(adyen_service),
            Arc::new(footprint_service)
        ));

        let error = wallet_engine.clone().match_card(
            &user,
            &MatchRequest {
                reference_id: expected_reference_id.clone(),
            }
        ).await.expect_err("should throw error on match");

        assert_eq!(ErrorType::Unauthorized, error.error_type);
    }

    #[actix_web::test]
    async fn test_match_fails_to_find() {
        let cc_dao = MockCreditCardDaoTrait::new();
        let mut wca_dao = MockWalletCardAttemtDaoTrait::new();
        let mut w_dao = MockWalletDaoTrait::new();
        let adyen_service = MockAdyenChargeServiceTrait::new();
        let footprint_service = MockFootprintServiceTrait::new();

        let user = create_mock_user();

        let expected_reference_id = Uuid::new_v4().to_string();

        let wallet_card = create_mock_wallet();

        wca_dao.expect_find_by_reference_id()
            .times(1)
            .with(eq(expected_reference_id.clone()))
            .return_const(
                Err(DataError::new(ErrorType::NotFound, "record not found"))
            );

        wca_dao.expect_update_card()
            .times(0);


        w_dao.expect_insert_card()
            .times(0);


        let wallet_engine = Arc::new(WalletService::new_with_services(
            Arc::new(cc_dao),
            Arc::new(wca_dao),
            Arc::new(w_dao),
            Arc::new(adyen_service),
            Arc::new(footprint_service)
        ));

        let error = wallet_engine.clone().match_card(
            &user,
            &MatchRequest {
                reference_id: expected_reference_id.clone(),
            }
        ).await.expect_err("should throw error on match");

        assert_eq!(ErrorType::NotFound, error.error_type);
    }
}