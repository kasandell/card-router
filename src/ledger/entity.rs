use chrono::NaiveDateTime;
use crate::schema::{registered_transactions, outer_charge_ledger, inner_charge_ledger, transaction_ledger};
use diesel::{BoolExpressionMethods, Identifiable, Insertable, Queryable, Selectable};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::util::db;
use crate::wallet::entity::Wallet;
use diesel::prelude::*;
use crate::asa::request::AsaRequest;
use crate::error::data_error::DataError;
use crate::error::error_type::ErrorType;
use crate::ledger::constant::ChargeStatus;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionMetadata {
    pub memo: String,
    pub amount_cents: i32,
    pub mcc: String
}

#[derive(Clone, Debug, Insertable, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
#[diesel(table_name = registered_transactions)]
pub struct InsertableRegisteredTransaction<'a> {
    pub user_id: i32,
    //pub transaction_id: Uuid,
    pub memo: &'a str,
    pub amount_cents: i32,
    pub mcc: &'a str
}

#[derive(Clone, Debug, Identifiable, Serialize, Deserialize, Queryable, Selectable)]
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


#[derive(Clone, Debug, Identifiable, Serialize, Deserialize, Queryable, Selectable)]
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

#[derive(Clone, Debug, Insertable)]
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

#[derive(Clone, Debug, Identifiable, Serialize, Deserialize, Queryable, Selectable)]
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
    pub updated_at: NaiveDateTime
}

#[derive(Clone, Debug, Insertable, Serialize, Deserialize, Queryable, Selectable)]
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
}

#[derive(Clone, Debug, Identifiable, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(belongs_to(RegisteredTransaction))]
#[diesel(belongs_to(InnerChargeLedger))]
#[diesel(belongs_to(OuterChargeLedger))]
#[diesel(table_name = transaction_ledger)]
pub struct TransactionLedger {
    pub id: i32,
    pub registered_transaction_id: i32,
    pub inner_charge_ledger_id: i32,
    pub outer_charge_ledger_id: i32
}

#[derive(Clone, Debug, Insertable, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(belongs_to(RegisteredTransaction))]
#[diesel(belongs_to(InnerChargeLedger))]
#[diesel(belongs_to(OuterChargeLedger))]
#[diesel(table_name = transaction_ledger)]
pub struct InsertableTransactionLedger {
    pub registered_transaction_id: i32,
    pub inner_charge_ledger_id: i32,
    pub outer_charge_ledger_id: i32
}




impl RegisteredTransaction {
    pub async fn insert<'a>(transaction: &InsertableRegisteredTransaction<'a>) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let txn = diesel::insert_into(registered_transactions::table)
            .values(transaction)
            .get_result(&mut conn).await?;
        Ok(txn)
    }

    pub async fn get_by_transaction_id(id: &Uuid) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let txn = registered_transactions::table.filter(
            registered_transactions::transaction_id.eq(id)
        ).first::<RegisteredTransaction>(&mut conn).await?;
        Ok(txn)
    }

    pub async fn get(id: i32) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let txn = registered_transactions::table.filter(
            registered_transactions::id.eq(id)
        ).first::<RegisteredTransaction>(&mut conn).await?;
        Ok(txn)
    }

    #[cfg(test)]
    pub async fn delete(id: i32) -> Result<usize, DataError> {
        let mut conn = db::connection().await?;

        let res = diesel::delete(
            registered_transactions::table
                .filter(registered_transactions::id.eq(id))
        )
            .execute(&mut conn).await?;
        Ok(res)
    }

    #[cfg(test)]
    pub async fn delete_self(&self) -> Result<usize, DataError> {
        RegisteredTransaction::delete(self.id).await
    }

    #[cfg(test)]
    pub async fn delete_all() -> Result<usize, DataError> {
        let mut conn = db::connection().await?;

        let res = diesel::delete(
            registered_transactions::table
        )
            .execute(&mut conn).await?;
        Ok(res)
    }
}


impl InnerChargeLedger {
    pub async fn insert(transaction: &InsertableInnerChargeLedger) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let txn = diesel::insert_into(inner_charge_ledger::table)
            .values(transaction)
            .get_result(&mut conn).await?;
        Ok(txn)
    }

    pub async fn get_inner_charges_by_registered_transaction(registered_transaction: i32) -> Result<Vec<Self>, DataError> {
        let mut conn = db::connection().await?;
        let txns = inner_charge_ledger::table
            .filter(
                inner_charge_ledger::registered_transaction_id.eq(registered_transaction)
            )
            .load::<InnerChargeLedger>(&mut conn).await?;
        Ok(txns)
    }

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

    pub async fn get_by_id(id: i32) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let txn = inner_charge_ledger::table
            .filter(
                inner_charge_ledger::id.eq(id)
            )
            .first(&mut conn).await?;
        Ok(txn)
    }

    #[cfg(test)]
    pub async fn delete(id: i32) -> Result<usize, DataError> {
        let mut conn = db::connection().await?;

        let res = diesel::delete(
            inner_charge_ledger::table
                .filter(inner_charge_ledger::id.eq(id))
        )
            .execute(&mut conn).await?;
        Ok(res)
    }

    #[cfg(test)]
    pub async fn delete_self(&self) -> Result<usize, DataError> {
        InnerChargeLedger::delete(self.id).await
    }

    #[cfg(test)]
    pub async fn delete_all() -> Result<usize, DataError> {
        let mut conn = db::connection().await?;

        let res = diesel::delete(
            inner_charge_ledger::table
        )
            .execute(&mut conn).await?;
        Ok(res)
    }
}

impl OuterChargeLedger {
    pub async fn insert(transaction: &InsertableOuterChargeLedger) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let txn = diesel::insert_into(outer_charge_ledger::table)
            .values(transaction)
            .get_result(&mut conn).await?;
        Ok(txn)
    }

    pub async fn get_outer_charge_by_registered_transaction(registered_transaction: i32) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let txn = outer_charge_ledger::table
            .filter(
                outer_charge_ledger::registered_transaction_id.eq(registered_transaction)
            )
            .first(&mut conn).await?;
        Ok(txn)
    }

    pub async fn get_by_id(id: i32) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let txn = outer_charge_ledger::table
            .filter(
                outer_charge_ledger::id.eq(id)
            )
            .first(&mut conn).await?;
        Ok(txn)
    }

    #[cfg(test)]
    pub async fn delete(id: i32) -> Result<usize, DataError> {
        let mut conn = db::connection().await?;

        let res = diesel::delete(
            outer_charge_ledger::table
                .filter(outer_charge_ledger::id.eq(id))
        )
            .execute(&mut conn).await?;
        Ok(res)
    }

    #[cfg(test)]
    pub async fn delete_self(&self) -> Result<usize, DataError> {
        OuterChargeLedger::delete(self.id).await
    }

    #[cfg(test)]
    pub async fn delete_all() -> Result<usize, DataError> {
        let mut conn = db::connection().await?;

        let res = diesel::delete(
            outer_charge_ledger::table
        )
            .execute(&mut conn).await?;
        Ok(res)
    }
}

impl TransactionLedger {
    pub async fn insert(transaction: &InsertableTransactionLedger) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let txn = diesel::insert_into(transaction_ledger::table)
            .values(transaction)
            .get_result(&mut conn).await?;
        Ok(txn)
    }

    pub async fn get_by_registered_transaction_id(id: i32) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let txn = transaction_ledger::table
            .filter(
                transaction_ledger::registered_transaction_id.eq(id)
            )
            .first(&mut conn).await?;
        Ok(txn)
    }

    pub async fn get_by_id(id: i32) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let txn = transaction_ledger::table
            .filter(
                transaction_ledger::id.eq(id)
            )
            .first(&mut conn).await?;
        Ok(txn)
    }

    #[cfg(test)]
    pub async fn delete(id: i32) -> Result<usize, DataError> {
        let mut conn = db::connection().await?;

        let res = diesel::delete(
            transaction_ledger::table
                .filter(transaction_ledger::id.eq(id))
        )
            .execute(&mut conn).await?;
        Ok(res)
    }

    #[cfg(test)]
    pub async fn delete_self(&self) -> Result<usize, DataError> {
        TransactionLedger::delete(self.id).await
    }

}


impl TransactionMetadata {
    pub fn convert(request: &AsaRequest) -> Result<Self, DataError> {
        let error = DataError::new(ErrorType::BadRequest, "missing field");
        let merchant = request.merchant.clone().ok_or(error.clone())?;
        let descriptor = merchant.descriptor.clone().ok_or(error.clone())?;
        let mcc = merchant.mcc.clone().ok_or(error.clone())?;
        let amount = request.amount.ok_or(error.clone())?;
        Ok(
            TransactionMetadata {
                memo: descriptor,
                amount_cents: amount,
                mcc: mcc
            }
        )
    }
}