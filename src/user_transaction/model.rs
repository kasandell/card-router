use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::category::constant::Category;
use crate::user_transaction::entity::{InnerCardChargeWithDetail, TransactionWithDetail};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InnerCardChargeWithDetailModel {
    pub memo: String,
    pub amount_cents: i32,
    pub created_at: NaiveDateTime,
    pub public_id: Uuid
    // add public id
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionWithDetailModel {
    pub memo: String,
    pub amount_cents: i32,
    pub category: Option<String>,
    pub credit_card_issuer: String,
    pub credit_card_type: String,
    pub credit_card_name: String,
    pub points_multiplier: Option<i32>,
    pub cashback_percentage_bips: Option<i32>,
    pub created_at: NaiveDateTime,
    pub public_id: Uuid
}

impl From<InnerCardChargeWithDetail> for InnerCardChargeWithDetailModel {
    fn from(value: InnerCardChargeWithDetail) -> Self {
        InnerCardChargeWithDetailModel {
            memo: value.registered_transaction_memo,
            amount_cents: value.wallet_card_charge_amount_cents,
            created_at: value.wallet_card_charge_created_at,
            public_id: value.public_id
        }
    }
}

impl From<TransactionWithDetail> for TransactionWithDetailModel {
    fn from(value: TransactionWithDetail) -> Self {
        TransactionWithDetailModel {
            memo: value.registered_transaction_memo,
            amount_cents: value.registered_transaction_amount_cents,
            category: value.category_name,
            credit_card_issuer: value.credit_card_issuer_name,
            credit_card_type: value.credit_card_type_name,
            credit_card_name: value.credit_card_name,
            points_multiplier: value.rule_points_multiplier,
            cashback_percentage_bips: value.rule_cashback_percentage_bips,
            created_at: value.wallet_card_charge_created_at,
            public_id: value.public_id
        }
    }
}