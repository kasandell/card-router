use serde::{Deserialize, Serialize};
use crate::user_transaction::model::{InnerCardChargeWithDetailModel, TransactionWithDetailModel};

#[derive(Clone, Deserialize, Serialize)]
pub struct TransactionsForWalletCardResponse {
    pub transactions: Vec<InnerCardChargeWithDetailModel>
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TransactionsForUserResponse {
    pub transactions: Vec<TransactionWithDetailModel>
}