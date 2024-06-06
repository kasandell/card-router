use chrono::Utc;
use crate::common::model::TransactionMetadata;
use crate::charge::constant::ChargeStatus;
use crate::charge::model::{
    WalletCardChargeModel,
    PassthroughCardChargeModel,
    RegisteredTransactionModel as RegisteredTransaction,
    SuccessfulEndToEndChargeModel
};

pub fn create_mock_registered_transaction(
    metadata: &TransactionMetadata
) -> RegisteredTransaction {
    RegisteredTransaction {
        id: -1,
        user_id: 1,
        transaction_id: Default::default(),
        memo: metadata.memo.clone(),
        amount_cents: metadata.amount_cents,
        mcc: metadata.mcc.clone()
    }
}

pub fn default_transaction_metadata() -> TransactionMetadata {
    TransactionMetadata {
        amount_cents: 0,
        memo: "".to_string(),
        mcc: "7184".to_string()
    }
}

pub fn create_mock_failed_wallet_charge() -> WalletCardChargeModel {
    WalletCardChargeModel {
        id: 1,
        registered_transaction_id: 1,
        user_id: 1,
        wallet_card_id: 1,
        amount_cents: 0,
        status: ChargeStatus::Fail,
        is_success: None,
        created_at: Utc::now().naive_utc(),
        rule_id: None
    }
}

pub fn create_mock_success_wallet_charge() -> WalletCardChargeModel {
    WalletCardChargeModel {
        id: 1,
        registered_transaction_id: 1,
        user_id: 1,
        wallet_card_id: 1,
        amount_cents: 0,
        status: ChargeStatus::Success,
        is_success: Some(true),
        created_at: Utc::now().naive_utc(),
        rule_id: None,
    }
}

pub fn create_mock_failed_passthrough_card_charge() -> PassthroughCardChargeModel {
    PassthroughCardChargeModel {
        id: 1,
        registered_transaction_id: 1,
        user_id: 1,
        passthrough_card_id: 1,
        amount_cents: 0,
        status: ChargeStatus::Fail,
        is_success: None,
        created_at: Utc::now().naive_utc(),
    }
}

pub fn create_mock_success_passthrough_card_charge() -> PassthroughCardChargeModel {
    PassthroughCardChargeModel {
        id: 1,
        registered_transaction_id: 1,
        user_id: 1,
        passthrough_card_id: 1,
        amount_cents: 0,
        status: ChargeStatus::Success,
        is_success: Some(true),
        created_at: Utc::now().naive_utc(),
    }
}

pub fn create_mock_full_transaction() -> SuccessfulEndToEndChargeModel {
    SuccessfulEndToEndChargeModel {
        id: 1,
        registered_transaction_id: 1,
        inner_charge_ledger_id: 1,
        outer_charge_ledger_id: 1,
    }
}