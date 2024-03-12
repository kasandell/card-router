// TODO: everything needs a rewrite
#[cfg(test)]
mod tests {
    use actix_web;
    use std::sync::Arc;
    use mockall::predicate::eq;
    use uuid::Uuid;
    use crate::adyen::checkout::service::MockAdyenChargeServiceTrait;
    use crate::api_error::ApiError;
    use crate::credit_card_type::dao::MockCreditCardDaoTrait;
    use crate::data_error::DataError;
    use crate::error_type::ErrorType;
    use crate::test_helper::{
        credit_card::create_mock_credit_card,
        wallet::create_mock_wallet,
        user::create_mock_user
    };
    use crate::test_helper::wallet::create_mock_wallet_attempt;

    use crate::wallet::constant::WalletCardAttemptStatus;
    use crate::wallet::dao::{MockWalletCardAttemtDaoTrait, MockWalletDaoTrait};
    use crate::wallet::service::WalletService;
    use crate::wallet::request::{MatchAttemptRequest, RegisterAttemptRequest};

    const USER_ID: i32 = 1;
    const CREDIT_CARD_ID: i32 = 1;
    const CREDIT_CARD_PUBLIC_ID: Uuid = Uuid::from_u128(0x9cb4cf49_5c3d_4647_83b0_8f3515da7be1);
    const CREDIT_CARD_NAME: &str = "Sapphire Reserve";

    #[actix_web::test]
    async fn test_register_attempt() {
        let mut cc_dao = MockCreditCardDaoTrait::new();
        let mut wca_dao = MockWalletCardAttemtDaoTrait::new();
        let mut w_dao = MockWalletDaoTrait::new();
        let mut adyen_service = MockAdyenChargeServiceTrait::new();

        let cc = create_mock_credit_card(CREDIT_CARD_NAME);
        let mut wca = create_mock_wallet_attempt();

        let user = create_mock_user();

        let expected_reference_id = Uuid::new_v4().to_string();
        wca.expected_reference_id = expected_reference_id.clone();
        let expected_reference_id_clone = expected_reference_id.clone();

        wca_dao.expect_insert()
            .times(1)
            .withf(move |insert_request| {
                insert_request.user_id == USER_ID
                && insert_request.credit_card_id == CREDIT_CARD_ID
                && insert_request.expected_reference_id == expected_reference_id_clone
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
        ));

        let wca_ret = wallet_engine.clone().attempt_register_new_attempt(
            &user,
            &RegisterAttemptRequest {
                expected_reference_id: expected_reference_id.clone(),
                credit_card_type_public_id: CREDIT_CARD_PUBLIC_ID,
            }
        ).await.expect("no error");

        assert_eq!(wca, wca_ret);
    }

    #[actix_web::test]
    async fn test_register_attempt_fails() {
        let mut cc_dao = MockCreditCardDaoTrait::new();
        let mut wca_dao = MockWalletCardAttemtDaoTrait::new();
        let mut w_dao = MockWalletDaoTrait::new();
        let mut adyen_service = MockAdyenChargeServiceTrait::new();

        let cc = create_mock_credit_card(CREDIT_CARD_NAME);
        let wca = create_mock_wallet_attempt();
        let user = create_mock_user();

        let expected_reference_id: String = Uuid::new_v4().to_string();
        let expected_reference_id_clone = expected_reference_id.clone();

        wca_dao.expect_insert()
            .times(1)
            .withf(move |insert_request| {
                insert_request.user_id == USER_ID
                    && insert_request.credit_card_id == CREDIT_CARD_ID
                    && insert_request.expected_reference_id == expected_reference_id_clone
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
            Arc::new(adyen_service)
        ));

        let err: ApiError = wallet_engine.clone().attempt_register_new_attempt(
            &user,
            &RegisterAttemptRequest {
                expected_reference_id: expected_reference_id,
                credit_card_type_public_id: CREDIT_CARD_PUBLIC_ID,
            }
        ).await.expect_err("should return error");

        assert_eq!(ErrorType::InternalServerError, err.error_type);
    }

    #[actix_web::test]
    async fn test_register_attempt_several() {
        let mut cc_dao = MockCreditCardDaoTrait::new();
        let mut wca_dao = MockWalletCardAttemtDaoTrait::new();
        let mut w_dao = MockWalletDaoTrait::new();
        let mut adyen_service = MockAdyenChargeServiceTrait::new();

        let cc = create_mock_credit_card(CREDIT_CARD_NAME);
        let wca = create_mock_wallet_attempt();

        let user = create_mock_user();

        let expected_reference_id = Uuid::new_v4().to_string();
        let expected_reference_id_clone = expected_reference_id.clone();

        wca_dao.expect_insert()
            .times(2)
            .withf(move |insert_request| {
                insert_request.user_id == USER_ID
                    && insert_request.credit_card_id == CREDIT_CARD_ID
                    && insert_request.expected_reference_id == expected_reference_id_clone
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
            Arc::new(adyen_service)
        ));

        let wca_ret = wallet_engine.clone().attempt_register_new_attempt(
            &user,
            &RegisterAttemptRequest {
                expected_reference_id: expected_reference_id.clone(),
                credit_card_type_public_id: CREDIT_CARD_PUBLIC_ID,
            }
        ).await.expect("no error");

        assert_eq!(wca, wca_ret);

        let wca_ret2 = wallet_engine.clone().attempt_register_new_attempt(
            &user,
            &RegisterAttemptRequest {
                expected_reference_id: expected_reference_id.clone(),
                credit_card_type_public_id: CREDIT_CARD_PUBLIC_ID,
            }
        ).await.expect("no error");
        assert_eq!(wca, wca_ret2);

    }

    #[actix_web::test]
    async fn test_match_find() {
        let mut cc_dao = MockCreditCardDaoTrait::new();
        let mut wca_dao = MockWalletCardAttemtDaoTrait::new();
        let mut w_dao = MockWalletDaoTrait::new();
        let mut adyen_service = MockAdyenChargeServiceTrait::new();

        let cc = create_mock_credit_card(CREDIT_CARD_NAME);
        let wca = create_mock_wallet_attempt();


        let user = create_mock_user();

        let expected_reference_id = Uuid::new_v4().to_string();
        let expected_reference_id_clone = expected_reference_id.clone();

        let new_card_id = Uuid::new_v4().to_string();
        let new_card_id_clone = new_card_id.clone();
        let new_card_id_clone_2 = new_card_id.clone();
        let psp_id = Uuid::new_v4().to_string();
        let psp_id_clone = psp_id.clone();


        let wallet_card = create_mock_wallet();

        let mut matched = wca.clone();
        matched.id = 1;
        matched.credit_card_id = 1;
        matched.expected_reference_id = expected_reference_id.clone();
        matched.psp_id = Some(psp_id.clone());
        matched.recurring_detail_reference = Some(new_card_id.clone());
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
                && card_attempt.recurring_detail_reference == new_card_id_clone
                && card_attempt.psp_id == psp_id_clone
                &&  card_attempt.status == WalletCardAttemptStatus::Matched

            })
            .return_const(
                Ok(matched.clone())
            );

        w_dao.expect_insert_card()
            .times(1)
            .withf(move |new_card| {
                new_card.user_id == USER_ID
                && new_card.payment_method_id == new_card_id_clone_2
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
            Arc::new(adyen_service)
        ));

        let created_card = wallet_engine.clone().attempt_match(
            &MatchAttemptRequest {
                merchant_reference_id: expected_reference_id.clone(),
                original_psp_reference: psp_id.clone(),
                psp_reference: new_card_id.clone()
            }
        ).await.expect("should be ok");

        assert_eq!(created_card, wallet_card);
    }

    #[actix_web::test]
    async fn test_match_fails_already_matched() {
        let mut cc_dao = MockCreditCardDaoTrait::new();
        let mut wca_dao = MockWalletCardAttemtDaoTrait::new();
        let mut w_dao = MockWalletDaoTrait::new();
        let mut adyen_service = MockAdyenChargeServiceTrait::new();

        let cc = create_mock_credit_card(CREDIT_CARD_NAME);

        let user = create_mock_user();

        let expected_reference_id = Uuid::new_v4().to_string();

        let new_card_id = Uuid::new_v4().to_string();
        let psp_id = Uuid::new_v4().to_string();

        let mut matched = create_mock_wallet_attempt();
        matched.id = 1;
        matched.credit_card_id = 1;
        matched.expected_reference_id = expected_reference_id.clone();
        matched.psp_id = Some(psp_id.clone());
        matched.recurring_detail_reference = Some(new_card_id.clone());
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
            Arc::new(adyen_service)
        ));

        let error = wallet_engine.clone().attempt_match(
            &MatchAttemptRequest {
                merchant_reference_id: expected_reference_id.clone(),
                original_psp_reference: psp_id.clone(),
                psp_reference: new_card_id.clone()
            }
        ).await.expect_err("should throw error on match");

        assert_eq!(ErrorType::Conflict, error.error_type);
    }

    #[actix_web::test]
    async fn test_match_fails_to_find() {
        let mut cc_dao = MockCreditCardDaoTrait::new();
        let mut wca_dao = MockWalletCardAttemtDaoTrait::new();
        let mut w_dao = MockWalletDaoTrait::new();
        let mut adyen_service = MockAdyenChargeServiceTrait::new();

        let user = create_mock_user();

        let expected_reference_id = Uuid::new_v4().to_string();
        let new_card_id = Uuid::new_v4().to_string();
        let psp_id = Uuid::new_v4().to_string();

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
            Arc::new(adyen_service)
        ));

        let error = wallet_engine.clone().attempt_match(
            &MatchAttemptRequest {
                merchant_reference_id: expected_reference_id.clone(),
                original_psp_reference: psp_id.clone(),
                psp_reference: new_card_id.clone()
            }
        ).await.expect_err("should throw error on match");

        assert_eq!(ErrorType::NotFound, error.error_type);
    }
}