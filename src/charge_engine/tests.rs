

#[cfg(test)]
mod tests {
    use adyen_checkout::models::payment_response::ResultCode;
    use adyen_checkout::models::PaymentResponse;
    use crate::wallet::entity::Wallet;
    use crate::user::entity::User;
    use crate::adyen_service::checkout::error::Error;
    use crate::charge_engine::engine::Engine;
    use uuid::Uuid;
    use crate::adyen_service::checkout::service::*;

    #[actix_web::test]
    async fn test_single_charge_fails_on_error() {
        let mut chargeService = MockAdyenChargeServiceTrait::new();
        chargeService.expect_charge_card_on_file()
            .return_const(
                Err(Error::new("test_error".to_string()))
            );

        let engine = Engine::new_with_service(Box::new(chargeService));
        let res = engine.charge_card_with_cleanup(
            Uuid::new_v4(),
            &Wallet::create_test_wallet(
                1,
                1,
                1
            ),
            &User::create_test_user(
                1
            ),
            0,
            "7184",
            "Test charge"
        ).await.expect("NO error");
        assert!(!res);
    }

    #[actix_web::test]
    async fn test_single_charge_succeeds() {
        let mut charge_service = MockAdyenChargeServiceTrait::new();
        let mut resp = PaymentResponse::new();
        resp.result_code = Some(ResultCode::Authorised);
        charge_service.expect_charge_card_on_file()
            .return_const(
                Ok(
                    resp
                )
            );

        let engine = Engine::new_with_service(Box::new(charge_service));
        let res = engine.charge_card_with_cleanup(
            Uuid::new_v4(),
            &Wallet::create_test_wallet(
                1,
                1,
                1
            ),
            &User::create_test_user(
                1
            ),
            0,
            "7184",
            "Test charge"
        ).await.expect("NO error");
        assert!(res);
    }
}