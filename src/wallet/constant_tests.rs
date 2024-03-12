#[cfg(test)]
mod test {
    use crate::wallet::constant::WalletCardAttemptStatus;

    #[test]
    pub fn test_wallet_card_attempt_status_serializes() {
        assert_eq!("FAILED", WalletCardAttemptStatus::Failed.to_string());
        assert_eq!("PENDING", WalletCardAttemptStatus::Pending.to_string());
        assert_eq!("MATCHED", WalletCardAttemptStatus::Matched.to_string());
    }
}