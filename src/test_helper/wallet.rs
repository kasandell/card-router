use crate::wallet::entity::{Wallet, WalletCardAttempt};

pub fn create_mock_wallet() -> Wallet {
    Wallet {
        id: 0,
        public_id: Default::default(),
        user_id: 1,
        payment_method_id: "".to_string(),
        created_at: Default::default(),
        updated_at: Default::default(),
        credit_card_id: 0,
        wallet_card_attempt_id: 0,
    }
}

pub fn create_mock_wallet_attempt() -> WalletCardAttempt {
    WalletCardAttempt {
        id: 0,
        public_id: Default::default(),
        user_id: 1,
        credit_card_id: 1,
        expected_reference_id: "".to_string(),
        psp_id: None,
        status: "".to_string(),
        recurring_detail_reference: None,
        created_at: Default::default(),
        updated_at: Default::default(),
    }
}