use chrono::NaiveDateTime;
use uuid::Uuid;
use crate::category::constant::Category;
use crate::ledger::constant::ChargeStatus;
use crate::ledger::entity::{InnerChargeLedger, OuterChargeLedger, RegisteredTransaction, TransactionLedger};

#[derive(Clone, Debug)]
pub struct RegisteredTransactionModel {
    pub id: i32,
    pub user_id: i32,
    pub transaction_id: Uuid,
    pub memo: String,
    pub amount_cents: i32,
    pub mcc: String
}

#[derive(Clone, Debug)]
pub struct OuterChargeLedgerModel {
    pub id: i32,
    pub registered_transaction_id: i32,
    pub user_id: i32,
    pub passthrough_card_id: i32,
    pub amount_cents: i32,
    pub status: ChargeStatus,
    pub is_success: Option<bool>,
    pub created_at: NaiveDateTime,
}


#[derive(Clone, Debug)]
pub struct InnerChargeLedgerModel {
    pub id: i32,
    pub registered_transaction_id: i32,
    pub user_id: i32,
    pub wallet_card_id: i32,
    pub amount_cents: i32,
    pub status: ChargeStatus,
    pub is_success: Option<bool>,
    pub created_at: NaiveDateTime,
    pub rule_id: Option<i32>,
}

#[derive(Clone, Debug)]
pub struct TransactionLedgerModel {
    pub id: i32,
    pub registered_transaction_id: i32,
    pub inner_charge_ledger_id: i32,
    pub outer_charge_ledger_id: i32,
    pub rule_id: Option<i32>,
}



impl From<RegisteredTransaction> for RegisteredTransactionModel {
    fn from(value: RegisteredTransaction) -> Self {
        RegisteredTransactionModel {
            id: value.id,
            user_id: value.user_id,
            transaction_id: value.transaction_id,
            memo: value.memo,
            amount_cents: value.amount_cents,
            mcc: value.mcc
        }
    }
}

impl From<OuterChargeLedger> for OuterChargeLedgerModel {
    fn from(value: OuterChargeLedger) -> Self {
        OuterChargeLedgerModel {
            id: value.id,
            registered_transaction_id: value.registered_transaction_id,
            user_id: value.user_id,
            passthrough_card_id: value.passthrough_card_id,
            amount_cents: value.amount_cents,
            status: value.status,
            is_success: value.is_success,
            created_at: value.created_at
        }
    }
}

impl From<InnerChargeLedger> for InnerChargeLedgerModel {
    fn from(value: InnerChargeLedger) -> Self {
        InnerChargeLedgerModel {
            id: value.id,
            registered_transaction_id: value.registered_transaction_id,
            user_id: value.user_id,
            wallet_card_id: value.wallet_card_id,
            amount_cents: value.amount_cents,
            status: value.status,
            is_success: value.is_success,
            created_at: value.created_at,
            rule_id: value.rule_id,
        }
    }
}

impl From<TransactionLedger> for TransactionLedgerModel {
    fn from(value: TransactionLedger) -> Self {
        TransactionLedgerModel {
            id: value.id,
            registered_transaction_id: value.registered_transaction_id,
            inner_charge_ledger_id: value.inner_charge_ledger_id,
            outer_charge_ledger_id: value.outer_charge_ledger_id,
            rule_id: value.rule_id,
        }
    }
}
