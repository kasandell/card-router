use std::sync::Arc;
use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;
use crate::user::model::UserModel;
use crate::user_transaction::dao::{UserTransactionDao, UserTransactionDaoTrait};
use crate::user_transaction::error::UserTransactionError;
use crate::user_transaction::model::{InnerCardChargeWithDetailModel, TransactionWithDetailModel};
use crate::wallet::service::{WalletService, WalletServiceTrait};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait UserTransactionServiceTrait {
    async fn get_successful_transactions_for_user_with_detail(
        self: Arc<Self>,
        user: &UserModel
    ) -> Result<Vec<TransactionWithDetailModel>, UserTransactionError>;

    async fn get_successful_transactions_for_user_and_card_with_detail(
        self: Arc<Self>,
        user: &UserModel,
        wallet_public_id: &Uuid
    ) -> Result<Vec<InnerCardChargeWithDetailModel>, UserTransactionError>;
}

pub struct UserTransactionService {
    dao: Arc<dyn UserTransactionDaoTrait>,
    wallet_service: Arc<dyn WalletServiceTrait>,
}

impl UserTransactionService {
    pub fn new_with_services(wallet_service: Arc<dyn WalletServiceTrait>) -> Self {
        Self {
            dao: Arc::new(UserTransactionDao::new()),
            wallet_service: wallet_service.clone()
        }
    }
}

#[async_trait(?Send)]
impl UserTransactionServiceTrait for UserTransactionService {
    async fn get_successful_transactions_for_user_with_detail(
        self: Arc<Self>,
        user: &UserModel
    ) -> Result<Vec<TransactionWithDetailModel>, UserTransactionError> {
        let results = self.dao.clone().get_all_successful_transactions_by_user_id_with_detail(user.id).await?
            .into_iter().map(|e| TransactionWithDetailModel::from(e)).collect();
        Ok(results)
    }

    async fn get_successful_transactions_for_user_and_card_with_detail(
        self: Arc<Self>,
        user: &UserModel,
        wallet_public_id: &Uuid
    ) -> Result<Vec<InnerCardChargeWithDetailModel>, UserTransactionError> {
        let wallet = self.wallet_service.clone().find_by_public_id(wallet_public_id).await
            .map_err(|e| UserTransactionError::NotFound(e.into()))?;
        if user.id != wallet.user_id {
            return Err(UserTransactionError::Unauthorized("User is not owner of card".into()))
        }
        let results = self.dao.clone().get_successful_inner_charges_by_user_and_wallet_card_id(user.id, wallet.id).await?
            .into_iter().map(|e| InnerCardChargeWithDetailModel::from(e)).collect();
        Ok(results)
    }
}