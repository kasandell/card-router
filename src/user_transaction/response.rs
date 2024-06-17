use serde::{Deserialize, Serialize};
use crate::pagination::response::PaginationResponse;
use crate::user_transaction::model::{InnerCardChargeWithDetailModel, TransactionWithDetailModel};

#[derive(Clone, Deserialize, Serialize)]
pub struct TransactionsForWalletCardResponse {
    pub transactions: Vec<InnerCardChargeWithDetailModel>,
    pub pagination: PaginationResponse,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TransactionsForUserResponse {
    pub transactions: Vec<TransactionWithDetailModel>,
    pub pagination: PaginationResponse,
}
