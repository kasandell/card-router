use chrono::NaiveDateTime;
use diesel_async::{AsyncConnection, RunQueryDsl};
use diesel::prelude::*;
use diesel::{Queryable};
use uuid::Uuid;
use crate::category::constant::Category;
use crate::error::data_error::DataError;
use crate::ledger::constant::ChargeStatus;
use crate::schema::{inner_charge_ledger, outer_charge_ledger, registered_transactions, rule, transaction_ledger, credit_card, credit_card_issuer, credit_card_type, category, wallet};
use crate::util::db;

#[derive(Queryable)]
pub struct InnerCardChargeWithDetail {
    pub transaction_ledger_id: i32,
    pub inner_charge_ledger_registered_transaction_id: i32,
    pub inner_charge_ledger_user_id: i32,
    pub inner_charge_ledger_wallet_card_id: i32,
    pub inner_charge_ledger_amount_cents: i32,
    pub inner_charge_ledger_status: ChargeStatus,
    pub inner_charge_ledger_is_success: Option<bool>,
    pub inner_charge_ledger_created_at: NaiveDateTime,
    pub inner_charge_ledger_updated_at: NaiveDateTime,
    pub inner_charge_ledger_rule_id: Option<i32>,
    pub registered_transaction_transaction_id: Uuid,
    pub registered_transaction_memo: String,
    pub registered_transaction_mcc: String
}




#[derive(Queryable)]
pub struct TransactionWithDetail {
    pub transaction_ledger_id: i32,
    pub inner_charge_ledger_user_id: i32,
    pub registered_transaction_memo: String,
    pub registered_transaction_amount_cents: i32,
    pub category_name: Option<String>, // we need better modeling than string
    pub credit_card_issuer_name: String,
    pub credit_card_type_name: String,
    pub credit_card_name: String,
    pub rule_points_multiplier: Option<i32>,
    pub rule_cashback_percentage_bips: Option<i32>,
    pub inner_charge_ledger_created_at: NaiveDateTime,
}


impl InnerCardChargeWithDetail {

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn get_successful_inner_charges_by_user_and_wallet_card_id(user_id: i32, wallet_card_id: i32) -> Result<Vec<InnerCardChargeWithDetail>, DataError> {
        let mut conn = db::connection().await?;
        let txns = inner_charge_ledger::table
            .inner_join(registered_transactions::table)
            .filter(
                inner_charge_ledger::user_id.eq(user_id)
                    .and(inner_charge_ledger::wallet_card_id.eq(wallet_card_id))
                    .and(inner_charge_ledger::is_success.eq(Some(true)))
            )
            .select((
                inner_charge_ledger::id, inner_charge_ledger::registered_transaction_id,
                inner_charge_ledger::user_id, inner_charge_ledger::wallet_card_id,
                inner_charge_ledger::amount_cents, inner_charge_ledger::status,
                inner_charge_ledger::is_success, inner_charge_ledger::created_at,
                inner_charge_ledger::updated_at, inner_charge_ledger::rule_id,
                registered_transactions::transaction_id, registered_transactions::memo,
                registered_transactions::mcc
            ))
            .order(inner_charge_ledger::id.desc())
            .load::<InnerCardChargeWithDetail>(&mut conn).await?;
        Ok(txns)
    }
}

impl TransactionWithDetail {
    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn get_by_user_id_with_detail(user_id: i32) -> Result<Vec<TransactionWithDetail>, DataError> {
        let mut conn = db::connection().await?;
        let txn = transaction_ledger::table
            .inner_join(
                registered_transactions::table
            ).inner_join(
            inner_charge_ledger::table
                .left_join(
                    // TODO: do i want to join thru rule or wallet
                    rule::table.inner_join(category::table)
                )
                .inner_join(
                    wallet::table
                        .inner_join(
                            credit_card::table
                                .inner_join(credit_card_type::table)
                                .inner_join(credit_card_issuer::table)
                        )
                )
        )

            .filter(
                inner_charge_ledger::user_id.eq(user_id)
            )
            .select((
                transaction_ledger::id, inner_charge_ledger::user_id, registered_transactions::memo, registered_transactions::amount_cents,
                category::name.nullable(), credit_card_issuer::name, credit_card_type::name, credit_card::name,
                rule::points_multiplier.nullable(), rule::cashback_percentage_bips.nullable(),
                inner_charge_ledger::created_at
            ))
            .order(transaction_ledger::id.desc())
            .load::<TransactionWithDetail>(&mut conn).await?;
        Ok(txn)
    }
}