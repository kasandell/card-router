use std::sync::Arc;
use async_trait::async_trait;
use crate::error::data_error::DataError;

#[cfg(test)]
use mockall::{automock, predicate::*};
use crate::user_transaction::entity::{InnerCardChargeWithDetail, TransactionWithDetail};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait UserTransactionDaoTrait {
    async fn get_all_successful_transactions_by_user_id_with_detail(self: Arc<Self>, user_id: i32) -> Result<Vec<TransactionWithDetail>, DataError>;
    async fn get_successful_inner_charges_by_user_and_wallet_card_id(self: Arc<Self>, user_id: i32, wallet_id: i32) -> Result<Vec<InnerCardChargeWithDetail>, DataError>;
}

pub struct UserTransactionDao {}

impl UserTransactionDao {
    pub fn new() -> Self {
        Self {

        }
    }
}

#[async_trait(?Send)]
impl UserTransactionDaoTrait for UserTransactionDao {

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn get_all_successful_transactions_by_user_id_with_detail(self: Arc<Self>, user_id: i32) -> Result<Vec<TransactionWithDetail>, DataError> {
        TransactionWithDetail::get_by_user_id_with_detail(user_id).await
    }
    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn get_successful_inner_charges_by_user_and_wallet_card_id(self: Arc<Self>, user_id: i32, wallet_id: i32) -> Result<Vec<InnerCardChargeWithDetail>, DataError> {
        InnerCardChargeWithDetail::get_successful_inner_charges_by_user_and_wallet_card_id(user_id, wallet_id).await

    }
}