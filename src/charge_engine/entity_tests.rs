#[cfg(test)]
mod tests {
    use adyen_checkout::models::payment_response::ResultCode;
    use crate::charge_engine::entity::{
            ChargeCardAttemptResult,
            ChargeEngineResult
    };

    #[actix_web::test]
    async fn test_result_code_to_charge_card_attempt_result() {
        assert_eq!(ChargeCardAttemptResult::from(ResultCode::Authorised), ChargeCardAttemptResult::Approved);
        assert_eq!(ChargeCardAttemptResult::from(ResultCode::Pending), ChargeCardAttemptResult::Approved);
        assert_eq!(ChargeCardAttemptResult::from(ResultCode::Received), ChargeCardAttemptResult::Approved);
        assert_eq!(ChargeCardAttemptResult::from(ResultCode::Success), ChargeCardAttemptResult::Approved);

        assert_eq!(ChargeCardAttemptResult::from(ResultCode::AuthenticationFinished), ChargeCardAttemptResult::Denied);
        assert_eq!(ChargeCardAttemptResult::from(ResultCode::AuthenticationNotRequired), ChargeCardAttemptResult::Denied);
        assert_eq!(ChargeCardAttemptResult::from(ResultCode::Cancelled), ChargeCardAttemptResult::Denied);
        assert_eq!(ChargeCardAttemptResult::from(ResultCode::ChallengeShopper), ChargeCardAttemptResult::Denied);
        assert_eq!(ChargeCardAttemptResult::from(ResultCode::Error), ChargeCardAttemptResult::Denied);
        assert_eq!(ChargeCardAttemptResult::from(ResultCode::IdentifyShopper), ChargeCardAttemptResult::Denied);
        assert_eq!(ChargeCardAttemptResult::from(ResultCode::PartiallyAuthorised), ChargeCardAttemptResult::Denied);
        assert_eq!(ChargeCardAttemptResult::from(ResultCode::PresentToShopper), ChargeCardAttemptResult::Denied);
        assert_eq!(ChargeCardAttemptResult::from(ResultCode::RedirectShopper), ChargeCardAttemptResult::Denied);
        assert_eq!(ChargeCardAttemptResult::from(ResultCode::Refused), ChargeCardAttemptResult::Denied);
    }

    #[actix_web::test]
    async fn test_bool_from_result_code() {
        assert!(bool::from(&ChargeCardAttemptResult::Approved));
        assert!(!bool::from(&ChargeCardAttemptResult::Denied));
        assert!(!bool::from(&ChargeCardAttemptResult::PartialCancelSucceeded));
        assert!(!bool::from(&ChargeCardAttemptResult::PartialCancelFailed));
    }
}