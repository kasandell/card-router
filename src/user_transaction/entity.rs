use chrono::NaiveDateTime;
use diesel_async::{AsyncConnection, RunQueryDsl};
use diesel::prelude::*;
use diesel::{Queryable};
use uuid::Uuid;
use crate::error::data_error::DataError;
use crate::charge::constant::ChargeStatus;
use crate::schema::{
    wallet_card_charge, passthrough_card_charge, registered_transaction, rule, successful_end_to_end_charge, credit_card, credit_card_issuer, credit_card_type, category, wallet
};
use crate::util::db;

#[derive(Queryable)]
pub struct InnerCardChargeWithDetail {
    pub successful_end_to_end_charge_id: i32,
    pub wallet_card_charge_registered_transaction_id: i32,
    pub wallet_card_charge_user_id: i32,
    pub wallet_card_charge_wallet_card_id: i32,
    pub wallet_card_charge_amount_cents: i32,
    pub wallet_card_charge_resolved_charge_status: ChargeStatus,
    pub wallet_card_charge_is_success: Option<bool>,
    pub wallet_card_charge_created_at: NaiveDateTime,
    pub wallet_card_charge_updated_at: NaiveDateTime,
    pub wallet_card_charge_rule_id: Option<i32>,
    pub registered_transaction_transaction_id: Uuid,
    pub registered_transaction_memo: String,
    pub registered_transaction_mcc: String
}




#[derive(Queryable)]
pub struct TransactionWithDetail {
    pub successful_end_to_end_charge_id: i32,
    pub wallet_card_charge_user_id: i32,
    pub registered_transaction_memo: String,
    pub registered_transaction_amount_cents: i32,
    pub category_name: Option<String>, // we need better modeling than string
    pub credit_card_issuer_name: String,
    pub credit_card_type_name: String,
    pub credit_card_name: String,
    pub rule_points_multiplier: Option<i32>,
    pub rule_cashback_percentage_bips: Option<i32>,
    pub wallet_card_charge_created_at: NaiveDateTime,
}


impl InnerCardChargeWithDetail {

    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn get_successful_inner_charges_by_user_and_wallet_card_id(user_id: i32, wallet_card_id: i32) -> Result<Vec<InnerCardChargeWithDetail>, DataError> {
        let mut conn = db::connection().await?;
        let txns = wallet_card_charge::table
            .inner_join(registered_transaction::table)
            .filter(
                wallet_card_charge::user_id.eq(user_id)
                    .and(wallet_card_charge::wallet_card_id.eq(wallet_card_id))
                    .and(wallet_card_charge::is_success.eq(Some(true)))
            )
            .select((
                wallet_card_charge::id, wallet_card_charge::registered_transaction_id,
                wallet_card_charge::user_id, wallet_card_charge::wallet_card_id,
                wallet_card_charge::amount_cents, wallet_card_charge::resolved_charge_status,
                wallet_card_charge::is_success, wallet_card_charge::created_at,
                wallet_card_charge::updated_at, wallet_card_charge::rule_id,
                registered_transaction::transaction_id, registered_transaction::memo,
                registered_transaction::mcc
            ))
            .order(wallet_card_charge::id.desc())
            .load::<InnerCardChargeWithDetail>(&mut conn).await?;
        Ok(txns)
    }
}

impl TransactionWithDetail {
    #[cfg_attr(feature="trace-detail", tracing::instrument)]
    pub async fn get_by_user_id_with_detail(user_id: i32) -> Result<Vec<TransactionWithDetail>, DataError> {
        let mut conn = db::connection().await?;
        let txn = successful_end_to_end_charge::table
            .inner_join(
                registered_transaction::table
            ).inner_join(
            wallet_card_charge::table
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
                wallet_card_charge::user_id.eq(user_id)
            )
            .select((
                successful_end_to_end_charge::id, wallet_card_charge::user_id, registered_transaction::memo, registered_transaction::amount_cents,
                category::name.nullable(), credit_card_issuer::name, credit_card_type::name, credit_card::name,
                rule::points_multiplier.nullable(), rule::cashback_percentage_bips.nullable(),
                wallet_card_charge::created_at
            ))
            .order(successful_end_to_end_charge::id.desc())
            .load::<TransactionWithDetail>(&mut conn).await?;
        Ok(txn)
    }
}