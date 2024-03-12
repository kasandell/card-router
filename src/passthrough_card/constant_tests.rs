#[cfg(test)]
mod tests {
    use crate::passthrough_card::constant::{PassthroughCardStatus, PassthroughCardType};

    #[test]
    fn test_conversion_passthrough_card_type() {
        assert_eq!(PassthroughCardType::Virtual.to_string(), "VIRTUAL".to_string());
        assert_eq!(PassthroughCardType::Physical.to_string(), "PHYSICAL".to_string());
        assert_eq!(PassthroughCardType::MerchantLocked.to_string(), "MERCHANT_LOCKED".to_string());
        assert_eq!(PassthroughCardType::SingleUse.to_string(), "SINGLE_USE".to_string());
    }

    #[test]
    fn test_conversion_passthrough_card_status() {
        assert_eq!(PassthroughCardStatus::Closed.to_string(), "CLOSED".to_string());
        assert_eq!(PassthroughCardStatus::Open.to_string(), "OPEN".to_string());
        assert_eq!(PassthroughCardStatus::Paused.to_string(), "PAUSED".to_string());
        assert_eq!(PassthroughCardStatus::PendingActivation.to_string(), "PENDING_ACTIVATION".to_string());
        assert_eq!(PassthroughCardStatus::PendingFulfillment.to_string(), "PENDING_FULFILLMENT".to_string());
    }

}