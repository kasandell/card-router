use chrono::NaiveDateTime;
use crate::ledger::constant::{MoneyMovementDirection, MoneyMovementType};
use crate::ledger::entity::{PendingPassthroughCardTransactionLedger, PendingWalletTransactionLedger, SettledPassthroughCardTransactionLedger, SettledWalletTransactionLedger};

#[derive(Debug, Clone)]
pub struct PendingPassthroughCardTransactionLedgerModel {
    pub id: i32,
    pub registered_transaction_id: i32,
    pub user_id: i32,
    pub passthrough_card_id: i32,
    pub money_movement_direction: MoneyMovementDirection,
    pub money_movement_type: MoneyMovementType,
    pub amount_cents: i32,
}

#[derive(Debug, Clone)]
pub struct SettledPassthroughCardTransactionLedgerModel {
    pub id: i32,
    pub registered_transaction_id: i32,
    pub user_id: i32,
    pub passthrough_card_id: i32,
    pub money_movement_direction: MoneyMovementDirection,
    pub money_movement_type: MoneyMovementType,
    pub amount_cents: i32,
}

#[derive(Debug, Clone)]
pub struct PendingWalletTransactionLedgerModel {
    pub id: i32,
    pub registered_transaction_id: i32,
    pub user_id: i32,
    pub wallet_id: i32,
    pub money_movement_direction: MoneyMovementDirection,
    pub money_movement_type: MoneyMovementType,
    pub amount_cents: i32,
}

#[derive(Debug, Clone)]
pub struct SettledWalletTransactionLedgerModel {
    pub id: i32,
    pub registered_transaction_id: i32,
    pub user_id: i32,
    pub wallet_id: i32,
    pub money_movement_direction: MoneyMovementDirection,
    pub money_movement_type: MoneyMovementType,
    pub amount_cents: i32,
}

impl From<PendingPassthroughCardTransactionLedger> for PendingPassthroughCardTransactionLedgerModel {
    fn from(value: PendingPassthroughCardTransactionLedger) -> Self {
        PendingPassthroughCardTransactionLedgerModel {
            id: value.id,
            registered_transaction_id: value.registered_transaction_id,
            user_id: value.user_id,
            passthrough_card_id: value.passthrough_card_id,
            money_movement_direction: value.money_movement_direction,
            money_movement_type: value.money_movement_type,
            amount_cents: value.amount_cents,
        }
    }
}

impl From<SettledPassthroughCardTransactionLedger> for SettledPassthroughCardTransactionLedgerModel {
    fn from(value: SettledPassthroughCardTransactionLedger) -> Self {
        SettledPassthroughCardTransactionLedgerModel {
            id: value.id,
            registered_transaction_id: value.registered_transaction_id,
            user_id: value.user_id,
            passthrough_card_id: value.passthrough_card_id,
            money_movement_direction: value.money_movement_direction,
            money_movement_type: value.money_movement_type,
            amount_cents: value.amount_cents,
        }
    }
}

impl From<PendingWalletTransactionLedger> for PendingWalletTransactionLedgerModel {
    fn from(value: PendingWalletTransactionLedger) -> Self {
        PendingWalletTransactionLedgerModel {
            id: value.id,
            registered_transaction_id: value.registered_transaction_id,
            user_id: value.user_id,
            wallet_id: value.wallet_id,
            money_movement_direction: value.money_movement_direction,
            money_movement_type: value.money_movement_type,
            amount_cents: value.amount_cents,
        }
    }
}

impl From<SettledWalletTransactionLedger> for SettledWalletTransactionLedgerModel {
    fn from(value: SettledWalletTransactionLedger) -> Self {
        SettledWalletTransactionLedgerModel {
            id: value.id,
            registered_transaction_id: value.registered_transaction_id,
            user_id: value.user_id,
            wallet_id: value.wallet_id,
            money_movement_direction: value.money_movement_direction,
            money_movement_type: value.money_movement_type,
            amount_cents: value.amount_cents,
        }
    }
}