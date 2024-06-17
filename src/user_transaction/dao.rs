use std::sync::Arc;
use async_trait::async_trait;
use crate::error::data_error::DataError;

use uuid::Uuid;
use crate::user_transaction::entity::{InnerCardChargeWithDetail, TransactionWithDetail};

#[async_trait(?Send)]
pub trait UserTransactionDaoTrait {
    async fn get_transaction_id_by_public_id(self: Arc<Self>, public_id: &Uuid) -> Result<i32, DataError>;
    async fn get_all_successful_transactions_by_user_id_with_detail(self: Arc<Self>, user_id: i32) -> Result<Vec<TransactionWithDetail>, DataError>;
    async fn get_all_successful_transactions_by_user_id_with_detail_paginated(
        self: Arc<Self>,
        user_id: i32,
        after_id: Option<i32>,
        limit: i32,
    ) -> Result<Vec<TransactionWithDetail>, DataError>;
    async fn get_inner_charge_id_by_public_id(self: Arc<Self>, public_id: &Uuid) -> Result<i32, DataError>;
    async fn get_successful_inner_charges_by_user_and_wallet_card_id(self: Arc<Self>, user_id: i32, wallet_id: i32) -> Result<Vec<InnerCardChargeWithDetail>, DataError>;
    async fn get_successful_inner_charges_by_user_and_wallet_card_id_paginated(
        self: Arc<Self>,
        user_id: i32,
        wallet_id: i32,
        after_id: Option<i32>,
        limit: i32,
    ) -> Result<Vec<InnerCardChargeWithDetail>, DataError>;
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


    async fn get_transaction_id_by_public_id(self: Arc<Self>, public_id: &Uuid) -> Result<i32, DataError> {
        TransactionWithDetail::get_id_by_public_id(&public_id).await
    }

    async fn get_inner_charge_id_by_public_id(self: Arc<Self>, public_id: &Uuid) -> Result<i32, DataError> {
        InnerCardChargeWithDetail::get_id_by_public_id(public_id).await
    }
    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn get_all_successful_transactions_by_user_id_with_detail(self: Arc<Self>, user_id: i32) -> Result<Vec<TransactionWithDetail>, DataError> {
        TransactionWithDetail::get_by_user_id_with_detail(user_id).await
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn get_all_successful_transactions_by_user_id_with_detail_paginated(
        self: Arc<Self>,
        user_id: i32,
        after_id: Option<i32>,
        limit: i32,
    ) -> Result<Vec<TransactionWithDetail>, DataError> {
        TransactionWithDetail::get_by_user_id_with_detail_paginated(
            user_id,
            after_id,
            limit
        ).await

    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn get_successful_inner_charges_by_user_and_wallet_card_id(self: Arc<Self>, user_id: i32, wallet_id: i32) -> Result<Vec<InnerCardChargeWithDetail>, DataError> {
        InnerCardChargeWithDetail::get_successful_inner_charges_by_user_and_wallet_card_id(user_id, wallet_id).await
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn get_successful_inner_charges_by_user_and_wallet_card_id_paginated(
        self: Arc<Self>,
        user_id: i32,
        wallet_id: i32,
        after_id: Option<i32>,
        limit: i32,
    ) -> Result<Vec<InnerCardChargeWithDetail>, DataError> {
        InnerCardChargeWithDetail::get_successful_inner_charges_by_user_and_wallet_card_id_paginated(
            user_id,
            wallet_id,
            after_id,
            limit
        ).await
    }
}