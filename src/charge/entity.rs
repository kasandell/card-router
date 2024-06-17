use std::sync::Arc;
use chrono::NaiveDateTime;
use crate::schema::{passthrough_card_charge, wallet_card_charge, registered_transaction, registered_transaction_metadata, successful_end_to_end_charge, expected_wallet_charge_reference};
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
use crate::charge::constant::ChargeStatus;
use crate::util::transaction::Transaction;

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
#[diesel(table_name = registered_transaction)]
pub struct InsertableRegisteredTransaction<'a> {
    pub user_id: i32,
    pub memo: &'a str,
    pub amount_cents: i32,
    pub mcc: &'a str
}

#[derive(Debug, Identifiable, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = registered_transaction)]
pub struct RegisteredTransaction {
    pub id: i32,
    pub user_id: i32,
    pub transaction_id: Uuid,
    pub memo: String,
    pub amount_cents: i32,
    pub mcc: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(belongs_to(RegisteredTransaction))]
#[diesel(table_name = registered_transaction_metadata)]
pub struct RegisteredTransactionMetadata {
    pub registered_transaction_id: i32,
    pub body: String
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(belongs_to(RegisteredTransaction))]
#[diesel(table_name = registered_transaction_metadata)]
pub struct InsertableRegisteredTransactionMetadata<'a> {
    pub registered_transaction_id: i32,
    pub body: &'a str
}


#[derive(Debug, Identifiable, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(belongs_to(RegisteredTransaction))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(PassthroughCard))]
#[diesel(table_name = passthrough_card_charge)]
pub struct PassthroughCardCharge {
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
#[diesel(table_name = passthrough_card_charge)]
pub struct InsertablePassthroughCardCharge {
    pub registered_transaction_id: i32,
    pub user_id: i32,
    pub passthrough_card_id: i32,
    pub amount_cents: i32,
    pub status: ChargeStatus,
    pub is_success: Option<bool>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(belongs_to(RegisteredTransaction))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Wallet))]
#[diesel(table_name = expected_wallet_charge_reference)]
pub struct InsertableExpectedWalletChargeReference {
    pub registered_transaction_id: i32,
    pub user_id: i32,
    pub wallet_card_id: i32,
    pub amount_cents: i32,
}

#[derive(Debug, Identifiable, Serialize, Deserialize, Queryable, Selectable, Clone)]
#[diesel(belongs_to(RegisteredTransaction))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Wallet))]
#[diesel(table_name = expected_wallet_charge_reference)]
pub struct ExpectedWalletChargeReference {
    pub id: i32,
    pub registered_transaction_id: i32,
    pub reference_id: Uuid,
    pub user_id: i32,
    pub wallet_card_id: i32,
    pub amount_cents: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Identifiable, Serialize, Deserialize, Queryable, Selectable, Clone)]
#[diesel(belongs_to(RegisteredTransaction))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Wallet))]
#[diesel(table_name = wallet_card_charge)]
pub struct WalletCardCharge {
    pub id: i32,
    pub registered_transaction_id: i32,
    pub user_id: i32,
    pub wallet_card_id: i32,
    pub amount_cents: i32,
    pub rule_id: Option<i32>,
    pub expected_wallet_charge_reference_id: i32,
    pub resolved_charge_status: ChargeStatus,
    pub psp_reference: Option<String>,
    pub returned_reference: Option<String>,
    pub returned_charge_status: Option<String>,
    pub is_success: Option<bool>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub public_id: Uuid
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(belongs_to(RegisteredTransaction))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Wallet))]
#[diesel(table_name = wallet_card_charge)]
pub struct InsertableWalletCardCharge {
    pub registered_transaction_id: i32,
    pub user_id: i32,
    pub wallet_card_id: i32,
    pub amount_cents: i32,
    pub rule_id: Option<i32>,
    pub expected_wallet_charge_reference_id: i32,
    pub resolved_charge_status: ChargeStatus,
    pub psp_reference: Option<String>,
    pub returned_reference: Option<String>,
    pub returned_charge_status: Option<String>,
    pub is_success: Option<bool>,
}


#[derive(Debug, Identifiable, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(belongs_to(RegisteredTransaction))]
#[diesel(belongs_to(InnerChargeLedger))]
#[diesel(belongs_to(OuterChargeLedger))]
#[diesel(table_name = successful_end_to_end_charge)]
pub struct SuccessfulEndToEndCharge {
    pub id: i32,
    pub registered_transaction_id: i32,
    pub wallet_card_charge_id: i32,
    pub passthrough_card_charge_id: i32,
    pub public_id: Uuid
}

#[derive(Debug, Insertable, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(belongs_to(RegisteredTransaction))]
#[diesel(belongs_to(InnerChargeLedger))]
#[diesel(belongs_to(OuterChargeLedger))]
#[diesel(table_name = successful_end_to_end_charge)]
pub struct InsertableSuccessfulEndToEndCharge {
    pub registered_transaction_id: i32,
    pub wallet_card_charge_id: i32,
    pub passthrough_card_charge_id: i32
}



impl RegisteredTransaction {
    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn insert<'a>(database_transaction: &mut Transaction<'_, '_>, registered_transaction: &InsertableRegisteredTransaction<'a>) -> Result<Self, DataError> {
        let txn = diesel::insert_into(registered_transaction::table)
            .values(registered_transaction)
            .get_result(database_transaction).await?;
        Ok(txn)
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn get_by_transaction_id(id: &Uuid) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let txn = registered_transaction::table.filter(
            registered_transaction::transaction_id.eq(id)
        ).first::<RegisteredTransaction>(&mut conn).await?;
        Ok(txn)
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn get(id: i32) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let txn = registered_transaction::table.filter(
            registered_transaction::id.eq(id)
        ).first::<RegisteredTransaction>(&mut conn).await?;
        Ok(txn)
    }
}

impl ExpectedWalletChargeReference {
    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn insert<'a>(transaction: &mut Transaction<'_, '_>, reference: &InsertableExpectedWalletChargeReference) -> Result<Self, DataError> {
        let reference = diesel::insert_into(expected_wallet_charge_reference::table)
            .values(reference)
            .get_result::<Self>(transaction).await?;
        Ok(reference)
    }
}


impl WalletCardCharge {
    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn insert<'a>(transaction: &mut Transaction<'_, '_>, charge: &InsertableWalletCardCharge) -> Result<Self, DataError> {
        let txn = diesel::insert_into(wallet_card_charge::table)
            .values(charge)
            .get_result::<Self>(transaction).await?;
        Ok(txn)
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn get_wallet_card_charges_by_registered_transaction(registered_transaction: i32) -> Result<Vec<Self>, DataError> {
        let mut conn = db::connection().await?;
        let txns = wallet_card_charge::table
            .filter(
                wallet_card_charge::registered_transaction_id.eq(registered_transaction)
            )
            .load::<WalletCardCharge>(&mut conn).await?;
        Ok(txns)
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn get_successful_wallet_card_charge_by_registered_transaction(registered_transaction: i32) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let txn = wallet_card_charge::table
            .filter(
                wallet_card_charge::registered_transaction_id.eq(registered_transaction)
                    .and(
                        wallet_card_charge::is_success.eq(Some(true))
                    )
            )
            .first(&mut conn).await?;
        Ok(txn)

    }

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn get_by_id(id: i32) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let txn = wallet_card_charge::table
            .filter(
                wallet_card_charge::id.eq(id)
            )
            .first(&mut conn).await?;
        Ok(txn)
    }

}

impl PassthroughCardCharge {
    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn insert<'a>(transaction: &mut Transaction<'_, '_>, charge: &InsertablePassthroughCardCharge) -> Result<Self, DataError> {
            let txn = diesel::insert_into(passthrough_card_charge::table)
            .values(charge)
            .get_result(transaction).await?;
        Ok(txn)
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn get_outer_charge_by_registered_transaction(registered_transaction: i32) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let txn = passthrough_card_charge::table
            .filter(
                passthrough_card_charge::registered_transaction_id.eq(registered_transaction)
            )
            .first(&mut conn).await?;
        Ok(txn)
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn get_by_id(id: i32) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let txn = passthrough_card_charge::table
            .filter(
                passthrough_card_charge::id.eq(id)
            )
            .first(&mut conn).await?;
        Ok(txn)
    }
}

impl SuccessfulEndToEndCharge {
    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn insert<'a>(transaction: &mut Transaction<'_, '_>, charge: &InsertableSuccessfulEndToEndCharge) -> Result<Self, DataError> {
        let txn = diesel::insert_into(successful_end_to_end_charge::table)
            .values(charge)
            .get_result(transaction).await?;
        Ok(txn)
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn get_by_registered_transaction_id(id: i32) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let txn = successful_end_to_end_charge::table
            .filter(
                successful_end_to_end_charge::registered_transaction_id.eq(id)
            )
            .first(&mut conn).await?;
        Ok(txn)
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn get_by_id(id: i32) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let txn = successful_end_to_end_charge::table
            .filter(
                successful_end_to_end_charge::id.eq(id)
            )
            .first(&mut conn).await?;
        Ok(txn)
    }
}

