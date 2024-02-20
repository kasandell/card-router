use chrono::NaiveDateTime;
use crate::schema::{registered_transactions, outer_charge_ledger, inner_charge_ledger, transaction_ledger, wallet};
use diesel::{BoolExpressionMethods, Identifiable, Insertable, Queryable, RunQueryDsl, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::api_error::ApiError;
use crate::util::db;
use crate::wallet::entity::Wallet;
use diesel::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionMetadata {
    pub memo: String,
    pub amount_cents: i64,
    pub mcc: String
}

#[derive(Clone, Debug, Insertable, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
#[diesel(table_name = registered_transactions)]
pub struct InsertableRegisteredTransaction {
    pub user_id: i32,
    pub transaction_id: Uuid,
    pub memo: String,
    pub amount_cents: i64,
    pub mcc: String
}

#[derive(Clone, Debug, Identifiable, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = registered_transactions)]
pub struct RegisteredTransaction {
    pub id: i32,
    pub user_id: i32,
    pub transaction_id: Uuid,
    pub memo: String,
    pub amount_cents: i64,
    pub mcc: String
}


#[derive(Clone, Debug, Identifiable, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(belongs_to(RegisteredTransaction))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(PassthroughCard))]
#[diesel(table_name = outer_charge_ledger)]
pub struct OuterChargeLedger {
    pub id: i32,
    pub registered_transaction_id: Uuid,
    pub user_id: i32,
    pub passthrough_card_id: i32,
    pub amount_cents: i64,
    pub status: String,
    pub is_success: Option<bool>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Clone, Debug, Insertable)]
#[diesel(belongs_to(RegisteredTransaction))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(PassthroughCard))]
#[diesel(table_name = outer_charge_ledger)]
pub struct InsertableOuterChargeLedger {
    pub registered_transaction_id: Uuid,
    pub user_id: i32,
    pub passthrough_card_id: i32,
    pub amount_cents: i64,
    pub status: String,
    pub is_success: Option<bool>,
}

#[derive(Clone, Debug, Identifiable, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(belongs_to(RegisteredTransaction))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Wallet))]
#[diesel(table_name = inner_charge_ledger)]
pub struct InnerChargeLedger {
    pub id: i32,
    pub registered_transaction_id: Uuid,
    pub user_id: i32,
    pub wallet_card_id: i32,
    pub amount_cents: i64,
    pub status: String,
    pub is_success: Option<bool>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Clone, Debug, Insertable, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(belongs_to(RegisteredTransaction))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Wallet))]
#[diesel(table_name = inner_charge_ledger)]
pub struct InsertableInnerChargeLedger {
    pub registered_transaction_id: Uuid,
    pub user_id: i32,
    pub wallet_card_id: i32,
    pub amount_cents: i64,
    pub status: String,
    pub is_success: Option<bool>,
}


impl RegisteredTransaction {
    pub fn insert(transaction: InsertableRegisteredTransaction) -> Result<Self, ApiError> {
        let mut conn = db::connection()?;
        let txn = diesel::insert_into(registered_transactions::table)
            .values(transaction)
            .get_result(&mut conn)?;
        Ok(txn)
    }

    pub fn get_by_transaction_id(id: Uuid) -> Result<Self, ApiError> {
        let mut conn = db::connection()?;
        let txn = registered_transactions::table.filter(
            registered_transactions::transaction_id.eq(id)
        ).first::<RegisteredTransaction>(&mut conn)?;
        Ok(txn)
    }

    pub fn get(id: i32) -> Result<Self, ApiError> {
        let mut conn = db::connection()?;
        let txn = registered_transactions::table.filter(
            registered_transactions::id.eq(id)
        ).first::<RegisteredTransaction>(&mut conn)?;
        Ok(txn)
    }

    #[cfg(test)]
    pub fn delete(id: i32) -> Result<usize, ApiError> {
        let mut conn = db::connection()?;

        let res = diesel::delete(
            registered_transactions::table
                .filter(registered_transactions::id.eq(id))
        )
            .execute(&mut conn)?;
        Ok(res)
    }

    #[cfg(test)]
    pub fn delete_self(&self) -> Result<usize, ApiError> {
        RegisteredTransaction::delete(self.id)
    }
}


impl InnerChargeLedger {
    pub fn insert(transaction: InsertableInnerChargeLedger) -> Result<Self, ApiError> {
        let mut conn = db::connection()?;
        let txn = diesel::insert_into(inner_charge_ledger::table)
            .values(transaction)
            .get_result(&mut conn)?;
        Ok(txn)
    }

    pub fn get_inner_charges_by_registered_transaction(registered_transaction: Uuid) -> Result<Vec<Self>, ApiError> {
        let mut conn = db::connection()?;
        let txns = inner_charge_ledger::table
            .filter(
                inner_charge_ledger::registered_transaction_id.eq(registered_transaction)
            )
            .load::<InnerChargeLedger>(&mut conn)?;
        Ok(txns)
    }

    pub fn get_successful_inner_charge_by_registered_transaction(registered_transaction: Uuid) -> Result<Self, ApiError> {
        let mut conn = db::connection()?;
        let txn = inner_charge_ledger::table
            .filter(
                inner_charge_ledger::registered_transaction_id.eq(registered_transaction)
                    .and(
                        inner_charge_ledger::is_success.eq(Some(true))
                    )
            )
            .first(&mut conn)?;
        Ok(txn)

    }

    pub fn get_by_id(id: i32) -> Result<Self, ApiError> {
        let mut conn = db::connection()?;
        let txn = inner_charge_ledger::table
            .filter(
                inner_charge_ledger::id.eq(id)
            )
            .first(&mut conn)?;
        Ok(txn)
    }

    #[cfg(test)]
    pub fn delete(id: i32) -> Result<usize, ApiError> {
        let mut conn = db::connection()?;

        let res = diesel::delete(
            inner_charge_ledger::table
                .filter(inner_charge_ledger::id.eq(id))
        )
            .execute(&mut conn)?;
        Ok(res)
    }

    #[cfg(test)]
    pub fn delete_self(&self) -> Result<usize, ApiError> {
        InnerChargeLedger::delete(self.id)
    }
}

impl OuterChargeLedger {
    pub fn insert(transaction: InsertableOuterChargeLedger) -> Result<Self, ApiError> {
        let mut conn = db::connection()?;
        let txn = diesel::insert_into(outer_charge_ledger::table)
            .values(transaction)
            .get_result(&mut conn)?;
        Ok(txn)
    }

    pub fn get_outer_charge_by_registered_transaction(registered_transaction: Uuid) -> Result<Self, ApiError> {
        let mut conn = db::connection()?;
        let txn = outer_charge_ledger::table
            .filter(
                outer_charge_ledger::registered_transaction_id.eq(registered_transaction)
            )
            .first(&mut conn)?;
        Ok(txn)
    }

    pub fn get_by_id(id: i32) -> Result<Self, ApiError> {
        let mut conn = db::connection()?;
        let txn = outer_charge_ledger::table
            .filter(
                outer_charge_ledger::id.eq(id)
            )
            .first(&mut conn)?;
        Ok(txn)
    }

    #[cfg(test)]
    pub fn delete(id: i32) -> Result<usize, ApiError> {
        let mut conn = db::connection()?;

        let res = diesel::delete(
            outer_charge_ledger::table
                .filter(outer_charge_ledger::id.eq(id))
        )
            .execute(&mut conn)?;
        Ok(res)
    }

    #[cfg(test)]
    pub fn delete_self(&self) -> Result<usize, ApiError> {
        OuterChargeLedger::delete(self.id)
    }
}