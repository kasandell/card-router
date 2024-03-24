use chrono::Utc;
use uuid::Uuid;
use crate::error::data_error::DataError;
use crate::wallet::constant::WalletCardAttemptStatus;
use crate::wallet::model::{
    WalletModel as Wallet,
    WalletCardAttemptModel as WalletCardAttempt
};

pub fn create_mock_wallet() -> Wallet {
    Wallet {
        id: 1,
        public_id: Default::default(),
        user_id: 1,
        payment_method_id: "".to_string(),
        created_at: Default::default(),
        credit_card_id: 0,
        wallet_card_attempt_id: 0,
    }
}

pub fn create_mock_wallet_with_args(
    id: i32,
    user_id: i32,
    credit_card_id: i32
) -> Wallet {
    Wallet {
        id: id,
        public_id: Uuid::new_v4(),
        user_id: user_id,
        payment_method_id: Uuid::new_v4().to_string(),
        created_at: Utc::now().naive_utc(),
        credit_card_id: credit_card_id,
        wallet_card_attempt_id: 0
    }
}

pub fn create_mock_wallet_attempt() -> WalletCardAttempt {
    WalletCardAttempt {
        id: 1,
        public_id: Default::default(),
        user_id: 1,
        credit_card_id: 1,
        expected_reference_id: "".to_string(),
        status: WalletCardAttemptStatus::Pending,
        created_at: Default::default(),
    }
}