#[cfg(test)]
mod tests {
    use crate::asa::response::{
        AsaResponseResult,
        AvsResponseResult
    };
    use crate::charge_engine::entity::ChargeEngineResult;
    use serde_json;

    #[actix_web::test]
    async fn test_asa_from_str() {
        assert_eq!(AsaResponseResult::AccountInactive, AsaResponseResult::from("ACCOUNT_INACTIVE"));
        assert_eq!(AsaResponseResult::AvsInvalid, AsaResponseResult::from("AVS_INVALID"));
        assert_eq!(AsaResponseResult::CardClosed, AsaResponseResult::from("CARD_CLOSED"));
        assert_eq!(AsaResponseResult::CardPaused, AsaResponseResult::from("CARD_PAUSED"));
        assert_eq!(AsaResponseResult::InsufficientFunds, AsaResponseResult::from("INSUFFICIENT_FUNDS"));
        assert_eq!(AsaResponseResult::UnauthorizedMerchant, AsaResponseResult::from("UNAUTHORIZED_MERCHANT"));
        assert_eq!(AsaResponseResult::VelocityExceeded, AsaResponseResult::from("VELOCITY_EXCEEDED"));
        assert_eq!(AsaResponseResult::Approved, AsaResponseResult::from("APPROVED"));
        assert_eq!(AsaResponseResult::UnauthorizedMerchant, AsaResponseResult::from("WEIRD_CODE"));
    }

    #[actix_web::test]
    async fn test_asa_from_string() {
        assert_eq!(AsaResponseResult::AccountInactive, AsaResponseResult::from("ACCOUNT_INACTIVE".to_string()));
        assert_eq!(AsaResponseResult::AvsInvalid, AsaResponseResult::from("AVS_INVALID".to_string()));
        assert_eq!(AsaResponseResult::CardClosed, AsaResponseResult::from("CARD_CLOSED".to_string()));
        assert_eq!(AsaResponseResult::CardPaused, AsaResponseResult::from("CARD_PAUSED".to_string()));
        assert_eq!(AsaResponseResult::InsufficientFunds, AsaResponseResult::from("INSUFFICIENT_FUNDS".to_string()));
        assert_eq!(AsaResponseResult::UnauthorizedMerchant, AsaResponseResult::from("UNAUTHORIZED_MERCHANT".to_string()));
        assert_eq!(AsaResponseResult::VelocityExceeded, AsaResponseResult::from("VELOCITY_EXCEEDED".to_string()));
        assert_eq!(AsaResponseResult::Approved, AsaResponseResult::from("APPROVED".to_string()));
        assert_eq!(AsaResponseResult::UnauthorizedMerchant, AsaResponseResult::from("WEIRD_CODE".to_string()));
    }

    #[actix_web::test]
    async fn test_asa_to_string() {
        assert_eq!(String::from(AsaResponseResult::AccountInactive), "ACCOUNT_INACTIVE".to_string());
        assert_eq!(String::from(AsaResponseResult::AvsInvalid), "AVS_INVALID".to_string());
        assert_eq!(String::from(AsaResponseResult::CardClosed), "CARD_CLOSED".to_string());
        assert_eq!(String::from(AsaResponseResult::CardPaused), "CARD_PAUSED".to_string());
        assert_eq!(String::from(AsaResponseResult::InsufficientFunds), "INSUFFICIENT_FUNDS".to_string());
        assert_eq!(String::from(AsaResponseResult::UnauthorizedMerchant), "UNAUTHORIZED_MERCHANT".to_string());
        assert_eq!(String::from(AsaResponseResult::VelocityExceeded), "VELOCITY_EXCEEDED".to_string());
        assert_eq!(String::from(AsaResponseResult::Approved), "APPROVED".to_string());
    }

    #[actix_web::test]
    async fn test_asa_from_charge_engine_result() {
        assert_eq!(AsaResponseResult::CardPaused, AsaResponseResult::from(ChargeEngineResult::CardPaused));
        assert_eq!(AsaResponseResult::CardClosed, AsaResponseResult::from(ChargeEngineResult::CardClosed));
        assert_eq!(AsaResponseResult::UnauthorizedMerchant, AsaResponseResult::from(ChargeEngineResult::Denied));
        assert_eq!(AsaResponseResult::InsufficientFunds, AsaResponseResult::from(ChargeEngineResult::InsufficientFunds));
        assert_eq!(AsaResponseResult::Approved, AsaResponseResult::from(ChargeEngineResult::Approved));

    }

    #[actix_web::test]
    async fn test_deserialize_asa() {

    }

    #[actix_web::test]
    async fn test_serialize_asa() {
    }

}