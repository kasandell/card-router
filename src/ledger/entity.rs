use chrono::NaiveDateTime;
use crate::schema::{inner_charge_ledger, outer_charge_ledger, registered_transactions, rule, transaction_ledger, credit_card, credit_card_issuer, credit_card_type, category, wallet};
use diesel::{BoolExpressionMethods, Identifiable, Insertable, Queryable, Selectable};
use diesel::associations::HasTable;
use diesel_async::{AsyncConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::util::db;
use crate::wallet::model::WalletModel as Wallet;
use diesel::prelude::*;
use crate::category::constant::Category;
use crate::error::data_error::DataError;
use crate::ledger::constant::ChargeStatus;

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
#[diesel(table_name = registered_transactions)]
pub struct InsertableRegisteredTransaction<'a> {
    pub user_id: i32,
    //pub transaction_id: Uuid,
    pub memo: &'a str,
    pub amount_cents: i32,
    pub mcc: &'a str
}

#[derive(Debug, Identifiable, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = registered_transactions)]
pub struct RegisteredTransaction {
    pub id: i32,
    pub user_id: i32,
    pub transaction_id: Uuid,
    pub memo: String,
    pub amount_cents: i32,
    pub mcc: String
}


#[derive(Debug, Identifiable, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(belongs_to(RegisteredTransaction))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(PassthroughCard))]
#[diesel(table_name = outer_charge_ledger)]
pub struct OuterChargeLedger {
    pub id: i32,
    pub registered_transaction_id: i32,
    pub user_id: i32,
    pub passthrough_card_id: i32,
    pub amount_cents: i32,
    pub status: ChargeStatus,
    pub is_success: Option<bool>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Insertable)]
#[diesel(belongs_to(RegisteredTransaction))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(PassthroughCard))]
#[diesel(table_name = outer_charge_ledger)]
pub struct InsertableOuterChargeLedger {
    pub registered_transaction_id: i32,
    pub user_id: i32,
    pub passthrough_card_id: i32,
    pub amount_cents: i32,
    pub status: ChargeStatus,
    pub is_success: Option<bool>,
}

#[derive(Debug, Identifiable, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(belongs_to(RegisteredTransaction))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Wallet))]
#[diesel(table_name = inner_charge_ledger)]
pub struct InnerChargeLedger {
    pub id: i32,
    pub registered_transaction_id: i32,
    pub user_id: i32,
    pub wallet_card_id: i32,
    pub amount_cents: i32,
    pub status: ChargeStatus,
    pub is_success: Option<bool>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub rule_id: Option<i32>,
}

#[derive(Debug, Insertable, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(belongs_to(RegisteredTransaction))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Wallet))]
#[diesel(table_name = inner_charge_ledger)]
pub struct InsertableInnerChargeLedger {
    pub registered_transaction_id: i32,
    pub user_id: i32,
    pub wallet_card_id: i32,
    pub amount_cents: i32,
    pub status: ChargeStatus,
    pub is_success: Option<bool>,
    pub rule_id: Option<i32>,
}

#[derive(Debug, Identifiable, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(belongs_to(RegisteredTransaction))]
#[diesel(belongs_to(InnerChargeLedger))]
#[diesel(belongs_to(OuterChargeLedger))]
#[diesel(table_name = transaction_ledger)]
pub struct TransactionLedger {
    pub id: i32,
    pub registered_transaction_id: i32,
    pub inner_charge_ledger_id: i32,
    pub outer_charge_ledger_id: i32,
    pub rule_id: Option<i32>,
}

#[derive(Debug, Insertable, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(belongs_to(RegisteredTransaction))]
#[diesel(belongs_to(InnerChargeLedger))]
#[diesel(belongs_to(OuterChargeLedger))]
#[diesel(table_name = transaction_ledger)]
pub struct InsertableTransactionLedger {
    pub registered_transaction_id: i32,
    pub inner_charge_ledger_id: i32,
    pub outer_charge_ledger_id: i32,
    pub rule_id: Option<i32>,
}



impl RegisteredTransaction {
    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn insert<'a>(transaction: &InsertableRegisteredTransaction<'a>) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let res = conn.transaction::<_, diesel::result::Error, _>(|mut _conn| Box::pin(async move {
            let txn = diesel::insert_into(registered_transactions::table)
                .values(transaction)
                .get_result(&mut _conn).await?;
            Ok(txn)

        })).await?;
        Ok(res)
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn get_by_transaction_id(id: &Uuid) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let txn = registered_transactions::table.filter(
            registered_transactions::transaction_id.eq(id)
        ).first::<RegisteredTransaction>(&mut conn).await?;
        Ok(txn)
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn get(id: i32) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let txn = registered_transactions::table.filter(
            registered_transactions::id.eq(id)
        ).first::<RegisteredTransaction>(&mut conn).await?;
        Ok(txn)
    }
}


impl InnerChargeLedger {
    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn insert(transaction: &InsertableInnerChargeLedger) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let res = conn.transaction::<_, diesel::result::Error, _>(|mut _conn| Box::pin(async move {
            let txn = diesel::insert_into(inner_charge_ledger::table)
                .values(transaction)
                .get_result(&mut _conn).await?;
            Ok(txn)
        })).await?;
        Ok(res)
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn get_inner_charges_by_registered_transaction(registered_transaction: i32) -> Result<Vec<Self>, DataError> {
        let mut conn = db::connection().await?;
        let txns = inner_charge_ledger::table
            .filter(
                inner_charge_ledger::registered_transaction_id.eq(registered_transaction)
            )
            .load::<InnerChargeLedger>(&mut conn).await?;
        Ok(txns)
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn get_successful_inner_charge_by_registered_transaction(registered_transaction: i32) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let txn = inner_charge_ledger::table
            .filter(
                inner_charge_ledger::registered_transaction_id.eq(registered_transaction)
                    .and(
                        inner_charge_ledger::is_success.eq(Some(true))
                    )
            )
            .first(&mut conn).await?;
        Ok(txn)

    }

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn get_by_id(id: i32) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let txn = inner_charge_ledger::table
            .filter(
                inner_charge_ledger::id.eq(id)
            )
            .first(&mut conn).await?;
        Ok(txn)
    }

}

impl OuterChargeLedger {
    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn insert(transaction: &InsertableOuterChargeLedger) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let res = conn.transaction::<_, diesel::result::Error, _>(|mut _conn| Box::pin(async move {
            let txn = diesel::insert_into(outer_charge_ledger::table)
                .values(transaction)
                .get_result(&mut _conn).await?;
            Ok(txn)
        })).await?;
        Ok(res)
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn get_outer_charge_by_registered_transaction(registered_transaction: i32) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let txn = outer_charge_ledger::table
            .filter(
                outer_charge_ledger::registered_transaction_id.eq(registered_transaction)
            )
            .first(&mut conn).await?;
        Ok(txn)
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn get_by_id(id: i32) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let txn = outer_charge_ledger::table
            .filter(
                outer_charge_ledger::id.eq(id)
            )
            .first(&mut conn).await?;
        Ok(txn)
    }
}

impl TransactionLedger {
    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn insert(transaction: &InsertableTransactionLedger) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let res = conn.transaction::<_, diesel::result::Error, _>(|mut _conn| Box::pin(async move {
            let txn = diesel::insert_into(transaction_ledger::table)
                .values(transaction)
                .get_result(&mut _conn).await?;
            Ok(txn)
        })).await?;
        Ok(res)
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn get_by_registered_transaction_id(id: i32) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let txn = transaction_ledger::table
            .filter(
                transaction_ledger::registered_transaction_id.eq(id)
            )
            .first(&mut conn).await?;
        Ok(txn)
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn get_by_id(id: i32) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let txn = transaction_ledger::table
            .filter(
                transaction_ledger::id.eq(id)
            )
            .first(&mut conn).await?;
        Ok(txn)
    }
}

