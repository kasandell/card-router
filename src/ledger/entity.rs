use std::sync::Arc;
use chrono::NaiveDateTime;
use crate::schema::{
    pending_passthrough_card_transaction_ledger,
    settled_passthrough_card_transaction_ledger,
    pending_wallet_transaction_ledger,
    settled_wallet_transaction_ledger
};
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use diesel::associations::HasTable;
use diesel_async::{AsyncConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::error::data_error::DataError;
use crate::ledger::constant::{MoneyMovementDirection, MoneyMovementType};
use crate::util::transaction::Transaction;


#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug, Selectable, Clone, PartialEq)]
#[diesel(table_name = pending_passthrough_card_transaction_ledger)]
pub struct PendingPassthroughCardTransactionLedger {
    pub id: i32,
    pub registered_transaction_id: i32,
    pub user_id: i32,
    pub passthrough_card_id: i32,
    pub money_movement_direction: MoneyMovementDirection,
    pub money_movement_type: MoneyMovementType,
    pub amount_cents: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = pending_passthrough_card_transaction_ledger)]
pub struct InsertablePendingPassthroughCardTransactionLedger {
    pub registered_transaction_id: i32,
    pub user_id: i32,
    pub passthrough_card_id: i32,
    pub money_movement_direction: MoneyMovementDirection,
    pub money_movement_type: MoneyMovementType,
    pub amount_cents: i32,
}

#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug, Selectable, Clone, PartialEq)]
#[diesel(table_name = settled_passthrough_card_transaction_ledger)]
pub struct SettledPassthroughCardTransactionLedger {
    pub id: i32,
    pub registered_transaction_id: i32,
    pub user_id: i32,
    pub passthrough_card_id: i32,
    pub money_movement_direction: MoneyMovementDirection,
    pub money_movement_type: MoneyMovementType,
    pub amount_cents: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = settled_passthrough_card_transaction_ledger)]
pub struct InsertableSettledPassthroughCardTransactionLedger {
    pub registered_transaction_id: i32,
    pub user_id: i32,
    pub passthrough_card_id: i32,
    pub money_movement_direction: MoneyMovementDirection,
    pub money_movement_type: MoneyMovementType,
    pub amount_cents: i32,
}


#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug, Selectable, Clone, PartialEq)]
#[diesel(table_name = pending_wallet_transaction_ledger)]
pub struct PendingWalletTransactionLedger {
    pub id: i32,
    pub registered_transaction_id: i32,
    pub user_id: i32,
    pub wallet_id: i32,
    pub money_movement_direction: MoneyMovementDirection,
    pub money_movement_type: MoneyMovementType,
    pub amount_cents: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = pending_wallet_transaction_ledger)]
pub struct InsertablePendingWalletTransactionLedger {
    pub registered_transaction_id: i32,
    pub user_id: i32,
    pub wallet_id: i32,
    pub money_movement_direction: MoneyMovementDirection,
    pub money_movement_type: MoneyMovementType,
    pub amount_cents: i32,
}

#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug, Selectable, Clone, PartialEq)]
#[diesel(table_name = settled_wallet_transaction_ledger)]
pub struct SettledWalletTransactionLedger {
    pub id: i32,
    pub registered_transaction_id: i32,
    pub user_id: i32,
    pub wallet_id: i32,
    pub money_movement_direction: MoneyMovementDirection,
    pub money_movement_type: MoneyMovementType,
    pub amount_cents: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


#[derive(Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = settled_wallet_transaction_ledger)]
pub struct InsertableSettledWalletTransactionLedger {
    pub registered_transaction_id: i32,
    pub user_id: i32,
    pub wallet_id: i32,
    pub money_movement_direction: MoneyMovementDirection,
    pub money_movement_type: MoneyMovementType,
    pub amount_cents: i32,
}


impl PendingPassthroughCardTransactionLedger {
    pub async fn insert<'a>(transaction: Arc<Transaction<'a>>, ledger: &InsertablePendingPassthroughCardTransactionLedger) -> Result<Self, DataError> {
        let record = diesel::insert_into(pending_passthrough_card_transaction_ledger::table)
            .values(ledger)
            .get_result::<Self>(&mut transaction.lock()).await?;
        Ok(record)

    }
    pub async fn get<'a>(transaction: Arc<Transaction<'a>>, id: i32) -> Result<Self, DataError> {
        let record = pending_passthrough_card_transaction_ledger::table
            .filter(pending_passthrough_card_transaction_ledger::id.eq(id))
            .get_result::<Self>(&mut transaction.lock()).await?;
        Ok(record)

    }
}

impl SettledPassthroughCardTransactionLedger {
    pub async fn insert<'a>(transaction: Arc<Transaction<'a>>, ledger: &InsertableSettledPassthroughCardTransactionLedger) -> Result<Self, DataError> {
        let record = diesel::insert_into(settled_passthrough_card_transaction_ledger::table)
            .values(ledger)
            .get_result::<Self>(&mut transaction.lock()).await?;
        Ok(record)
    }
    pub async fn get<'a>(transaction: Arc<Transaction<'a>>, id: i32) -> Result<Self, DataError> {
        let record = settled_passthrough_card_transaction_ledger::table
            .filter(settled_passthrough_card_transaction_ledger::id.eq(id))
            .get_result::<Self>(&mut transaction.lock()).await?;
        Ok(record)
    }

}

impl PendingWalletTransactionLedger {
    pub async fn insert<'a>(transaction: Arc<Transaction<'a>>, ledger: &InsertablePendingWalletTransactionLedger) -> Result<Self, DataError> {
        let record = diesel::insert_into(pending_wallet_transaction_ledger::table)
            .values(ledger)
            .get_result::<Self>(&mut transaction.lock()).await?;
        Ok(record)

    }
    pub async fn get<'a>(transaction: Arc<Transaction<'a>>, id: i32) -> Result<Self, DataError> {
        let record = pending_wallet_transaction_ledger::table
            .filter(pending_wallet_transaction_ledger::id.eq(id))
            .get_result::<Self>(&mut transaction.lock()).await?;
        Ok(record)
    }

}

impl SettledWalletTransactionLedger {
    pub async fn insert<'a>(transaction: Arc<Transaction<'a>>, ledger: &InsertableSettledWalletTransactionLedger) -> Result<Self, DataError> {
        let record = diesel::insert_into(settled_wallet_transaction_ledger::table)
            .values(ledger)
            .get_result::<Self>(&mut transaction.lock()).await?;
        Ok(record)

    }
    pub async fn get<'a>(transaction: Arc<Transaction<'a>>, id: i32) -> Result<Self, DataError> {
        let record = settled_wallet_transaction_ledger::table
            .filter(settled_wallet_transaction_ledger::id.eq(id))
            .get_result::<Self>(&mut transaction.lock()).await?;
        Ok(record)
    }

}
