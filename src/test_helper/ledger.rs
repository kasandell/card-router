use chrono::Utc;
use crate::ledger::constant::ChargeStatus;
use crate::ledger::entity::{InnerChargeLedger, InsertableRegisteredTransaction, OuterChargeLedger, RegisteredTransaction, TransactionLedger, TransactionMetadata};
use crate::user::entity::User;

pub fn create_mock_registered_transaction(
    metadata: &TransactionMetadata
) -> RegisteredTransaction {
    RegisteredTransaction {
        id: 1,
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

pub fn create_mock_failed_inner_charge() -> InnerChargeLedger {
    InnerChargeLedger {
        id: 1,
        registered_transaction_id: 1,
        user_id: 1,
        wallet_card_id: 1,
        amount_cents: 0,
        status: ChargeStatus::Fail.as_str(),
        is_success: None,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc()
    }
}

pub fn create_mock_success_inner_charge() -> InnerChargeLedger {
    InnerChargeLedger {
        id: 1,
        registered_transaction_id: 1,
        user_id: 1,
        wallet_card_id: 1,
        amount_cents: 0,
        status: ChargeStatus::Success.as_str(),
        is_success: Some(true),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc()
    }
}

pub fn create_mock_failed_outer_charge() -> OuterChargeLedger {
    OuterChargeLedger {
        id: 1,
        registered_transaction_id: 1,
        user_id: 1,
        passthrough_card_id: 1,
        amount_cents: 0,
        status: ChargeStatus::Fail.as_str(),
        is_success: None,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc()
    }
}

pub fn create_mock_success_outer_charge() -> OuterChargeLedger {
    OuterChargeLedger {
        id: 1,
        registered_transaction_id: 1,
        user_id: 1,
        passthrough_card_id: 1,
        amount_cents: 0,
        status: ChargeStatus::Success.as_str(),
        is_success: Some(true),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc()
    }
}

pub fn create_mock_full_transaction() -> TransactionLedger {
    TransactionLedger {
        id: 1,
        registered_transaction_id: 1,
        inner_charge_ledger_id: 1,
        outer_charge_ledger_id: 1,
    }
}