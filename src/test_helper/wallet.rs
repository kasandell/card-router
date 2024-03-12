use chrono::Utc;
use uuid::Uuid;
use crate::error::data_error::DataError;
use crate::wallet::constant::WalletCardAttemptStatus;
use crate::wallet::entity::{InsertableCardAttempt, NewCard, Wallet, WalletCardAttempt};

pub fn create_mock_wallet() -> Wallet {
    Wallet {
        id: 1,
        public_id: Default::default(),
        user_id: 1,
        payment_method_id: "".to_string(),
        created_at: Default::default(),
        updated_at: Default::default(),
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
        updated_at: Utc::now().naive_utc(),
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
        psp_id: None,
        status: WalletCardAttemptStatus::Pending,
        recurring_detail_reference: None,
        created_at: Default::default(),
        updated_at: Default::default(),
    }
}

pub async fn create_test_wallet_in_db(
    user_id: i32,
    credit_card_id: i32
) -> Result<(Wallet, WalletCardAttempt), DataError> {
    let ca = WalletCardAttempt::insert(
        &InsertableCardAttempt {
            user_id: user_id,
            credit_card_id: credit_card_id,
            expected_reference_id: "test",
        }
    ).await?;
    let wallet = Wallet::insert_card(
        &NewCard {
            user_id: user_id,
            payment_method_id: "test",
            credit_card_id: credit_card_id,
            wallet_card_attempt_id: ca.id,

        }
    ).await?;
    Ok((wallet, ca))
}