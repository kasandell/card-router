pub enum WalletCardAttemptStatus {
    PENDING,
    FAILED,
    MATCHED
}

impl WalletCardAttemptStatus {
    pub fn from_str(status: &str) -> WalletCardAttemptStatus {
        match status {
            "PENDING" => WalletCardAttemptStatus::PENDING,
            "FAILED" => WalletCardAttemptStatus::FAILED,
            "MATCHED" => WalletCardAttemptStatus::MATCHED,
            _ => WalletCardAttemptStatus::PENDING
        }
    }

    pub fn as_str(&self) -> String {
        match self {
            WalletCardAttemptStatus::PENDING => "PENDING".to_string(),
            WalletCardAttemptStatus::FAILED => "FAILED".to_string(),
            WalletCardAttemptStatus::MATCHED => "MATCHED".to_string(),
        }
    }
}