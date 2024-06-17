use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use crate::pagination::constant::DEFAULT_PAGE_SIZE;
use crate::pagination::request::PaginationRequest;
use crate::pagination::response::PaginationResponse;
use crate::pagination::service::{PaginationService, PaginationServiceTrait};
use crate::user::model::UserModel;
use crate::user_transaction::dao::{UserTransactionDao, UserTransactionDaoTrait};
use crate::user_transaction::error::UserTransactionError;
use crate::user_transaction::model::{InnerCardChargeWithDetailModel, TransactionWithDetailModel};
use crate::wallet::service::{WalletService, WalletServiceTrait};

#[async_trait(?Send)]
pub trait UserTransactionServiceTrait {
    async fn get_successful_transactions_for_user_with_detail(
        self: Arc<Self>,
        user: &UserModel,
        pagination_request: &PaginationRequest
    ) -> Result<(Vec<TransactionWithDetailModel>, PaginationResponse), UserTransactionError>;

    async fn get_successful_transactions_for_user_and_card_with_detail(
        self: Arc<Self>,
        user: &UserModel,
        wallet_public_id: &Uuid,
        pagination_request: &PaginationRequest
    ) -> Result<(Vec<InnerCardChargeWithDetailModel>, PaginationResponse), UserTransactionError>;
}

pub struct UserTransactionService {
    dao: Arc<dyn UserTransactionDaoTrait>,
    wallet_service: Arc<dyn WalletServiceTrait>,
    pagination_service: Arc<dyn PaginationServiceTrait<Uuid> + Send>
}

impl UserTransactionService {
    pub fn new_with_services(
        wallet_service: Arc<dyn WalletServiceTrait>,
    ) -> Self {
        Self {
            dao: Arc::new(UserTransactionDao::new()),
            wallet_service: wallet_service.clone(),
            pagination_service: Arc::new(PaginationService::new()),
        }
    }
}

#[async_trait(?Send)]
impl UserTransactionServiceTrait for UserTransactionService {
    async fn get_successful_transactions_for_user_with_detail(
        self: Arc<Self>,
        user: &UserModel,
        pagination_request: &PaginationRequest,
    ) -> Result<(Vec<TransactionWithDetailModel>, PaginationResponse), UserTransactionError> {
        let limit = match &pagination_request.limit {
            Some(page_limit) => page_limit.clone(),
            None => DEFAULT_PAGE_SIZE
        } + 1;
        let mut after_id: Option<i32> = None;
        if let Some(cursor) = pagination_request.cursor.clone() {
            if cursor != "" {
                let paginateable_info = self.pagination_service.clone().decode_cursor_to_service_and_id(cursor)
                    .await.map_err(|e| UserTransactionError::UnexpectedError(e.into()))?;
                let public_id = paginateable_info.cursor_location;
                after_id = Some(self.dao.clone().get_transaction_id_by_public_id(&public_id).await?);
            }
        }
        let results: Vec<_> = self.dao.clone().get_all_successful_transactions_by_user_id_with_detail_paginated(
            user.id,
            after_id,
            limit
        ).await?
            .into_iter().map(|e| TransactionWithDetailModel::from(e)).collect();

        // at the end, no cursor needed
        if results.len() < limit as usize {
            return Ok((results.clone(), PaginationResponse { next_cursor: None }))
        }
        if let Some(last_res) = results.last() {
            let cursor = self.pagination_service.clone().encode_cursor_for_service_and_cursor(
                "user_transaction".to_string(),
                "successful_transactions".to_string(),
                last_res.public_id
            ).await.map_err(|e| UserTransactionError::UnexpectedError(e.into()))?;

            return Ok(
                (results[..results.len() - 1].to_vec(), PaginationResponse { next_cursor: Some(cursor) })
            )
        } else {
            return Ok((results, PaginationResponse { next_cursor: None }))
        }


    }

    async fn get_successful_transactions_for_user_and_card_with_detail(
        self: Arc<Self>,
        user: &UserModel,
        wallet_public_id: &Uuid,
        pagination_request: &PaginationRequest,
    ) -> Result<(Vec<InnerCardChargeWithDetailModel>, PaginationResponse), UserTransactionError> {
        let wallet = self.wallet_service.clone().find_by_public_id(wallet_public_id).await
            .map_err(|e| UserTransactionError::NotFound(e.into()))?;
        if user.id != wallet.user_id {
            return Err(UserTransactionError::Unauthorized("User is not owner of card".into()))
        }
        let limit = match &pagination_request.limit {
            Some(page_limit) => page_limit.clone(),
            None => DEFAULT_PAGE_SIZE
        } + 1;
        let mut after_id: Option<i32> = None;
        if let Some(cursor) = pagination_request.cursor.clone() {
            if cursor != "" {
                let paginateable_info = self.pagination_service.clone().decode_cursor_to_service_and_id(cursor)
                    .await.map_err(|e| UserTransactionError::UnexpectedError(e.into()))?;
                let public_id = paginateable_info.cursor_location;
                after_id = Some(self.dao.clone().get_inner_charge_id_by_public_id(&public_id).await?);
            }
        }
        let results: Vec<_> = self.dao.clone().get_successful_inner_charges_by_user_and_wallet_card_id_paginated(user.id, wallet.id, after_id, limit).await?
            .into_iter().map(|e| InnerCardChargeWithDetailModel::from(e)).collect();

        if results.len() < limit as usize {
            return Ok((results.clone(), PaginationResponse { next_cursor: None }))
        }
        if let Some(last_res) = results.last() {
            let cursor = self.pagination_service.clone().encode_cursor_for_service_and_cursor(
                "user_transaction".to_string(),
                "successful_transactions_for_card".to_string(),
                last_res.public_id
            ).await.map_err(|e| UserTransactionError::UnexpectedError(e.into()))?;

            return Ok(
                (results[..results.len() - 1].to_vec(), PaginationResponse { next_cursor: Some(cursor) })
            )
        } else {
            return Ok((results, PaginationResponse { next_cursor: None }))
        }
    }
}