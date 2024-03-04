use std::sync::{Arc, Mutex};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;

use uuid::Uuid;
use crate::passthrough_card::entity::PassthroughCard;
use crate::service_error::ServiceError;
use crate::transaction::constant::ChargeStatus;
use crate::transaction::dao::{TransactionDao, TransactionDaoTrait};
use crate::transaction::entity::{InnerChargeLedger, InsertableInnerChargeLedger, InsertableOuterChargeLedger, InsertableRegisteredTransaction, InsertableTransactionLedger, OuterChargeLedger, RegisteredTransaction, TransactionLedger, TransactionMetadata};
use crate::user::entity::User;
use crate::wallet::entity::Wallet;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait TransactionEngineTrait {
    async fn register_transaction_for_user(
        self: Arc<Self>,
        user: &User,
        metadata: &TransactionMetadata,
    ) -> Result<RegisteredTransaction, ServiceError>;

    async fn register_failed_inner_charge(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransaction,
        metadata: &TransactionMetadata,
        card: &Wallet
    ) -> Result<InnerChargeLedger, ServiceError>;

    async fn register_successful_inner_charge(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransaction,
        metadata: &TransactionMetadata,
        card: &Wallet
    ) -> Result<InnerChargeLedger, ServiceError>;

    async fn register_failed_outer_charge(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransaction,
        metadata: &TransactionMetadata,
        card: &PassthroughCard
    ) -> Result<OuterChargeLedger, ServiceError>;

    async fn register_successful_outer_charge(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransaction,
        metadata: &TransactionMetadata,
        card: &PassthroughCard
    ) -> Result<OuterChargeLedger, ServiceError>;

    async fn register_full_transaction(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransaction,
        successful_inner_charge: &InnerChargeLedger,
        successful_outer_charge: &OuterChargeLedger
    ) -> Result<TransactionLedger, ServiceError>;
}

pub struct Engine {
    dao: Arc<dyn TransactionDaoTrait>
}

impl Engine {
    pub fn new() -> Self {
        Self {
            dao: Arc::new(TransactionDao {})
        }
    }
}

#[async_trait(?Send)]
impl TransactionEngineTrait for Engine {
    async fn register_transaction_for_user(
        self: Arc<Self>,
        user: &User,
        metadata: &TransactionMetadata,
    ) -> Result<RegisteredTransaction, ServiceError> {
        let res = self.dao.clone().insert_registered_transaction(
            InsertableRegisteredTransaction {
                user_id: user.id,
                transaction_id: Uuid::new_v4(),
                memo: metadata.memo.to_string(),
                amount_cents: metadata.amount_cents,
                mcc: metadata.mcc.to_string()
            }
        ).await?;
        Ok(res)
    }

    async fn register_failed_inner_charge(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransaction,
        metadata: &TransactionMetadata,
        card: &Wallet
    ) -> Result<InnerChargeLedger, ServiceError> {
        // TODO: should do some verification somewhere that cards are associated with the correct user for the outer txn
        Ok(
            self.dao.clone().insert_inner_charge(
                InsertableInnerChargeLedger {
                    registered_transaction_id: registered_transaction.transaction_id,
                    user_id: registered_transaction.user_id,
                    wallet_card_id: card.id,
                    amount_cents: metadata.amount_cents,
                    status: ChargeStatus::Fail.as_str(),
                    is_success: None,
                }
            ).await?
        )
    }

    async fn register_successful_inner_charge(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransaction,
        metadata: &TransactionMetadata,
        card: &Wallet
    ) -> Result<InnerChargeLedger, ServiceError> {
        // TODO: should do some verification somewhere that cards are associated with the correct user for the outer txn
        Ok(
            self.dao.clone().insert_inner_charge(
                InsertableInnerChargeLedger {
                    registered_transaction_id: registered_transaction.transaction_id,
                    user_id: registered_transaction.user_id,
                    wallet_card_id: card.id,
                    amount_cents: metadata.amount_cents,
                    status: ChargeStatus::Success.as_str(),
                    is_success: Some(true),
                }
            ).await?
        )
    }

    async fn register_failed_outer_charge(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransaction,
        metadata: &TransactionMetadata,
        card: &PassthroughCard
    ) -> Result<OuterChargeLedger, ServiceError> {
        // TODO: do some assertions that everything is associated
        Ok(
            self.dao.clone().insert_outer_charge(
                InsertableOuterChargeLedger {
                    registered_transaction_id: registered_transaction.transaction_id,
                    user_id: registered_transaction.user_id,
                    passthrough_card_id: card.id,
                    amount_cents: metadata.amount_cents,
                    status: ChargeStatus::Fail.as_str(),
                    is_success: None,
                }
            ).await?
        )
    }

    async fn register_successful_outer_charge(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransaction,
        metadata: &TransactionMetadata,
        card: &PassthroughCard
    ) -> Result<OuterChargeLedger, ServiceError> {
        // TODO: do some assertions that everything is associated
        Ok(
            self.dao.clone().insert_outer_charge(
                InsertableOuterChargeLedger {
                    registered_transaction_id: registered_transaction.transaction_id,
                    user_id: registered_transaction.user_id,
                    passthrough_card_id: card.id,
                    amount_cents: metadata.amount_cents,
                    status: ChargeStatus::Success.as_str(),
                    is_success: Some(true),
                }
            ).await?
        )
    }

    async fn register_full_transaction(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransaction,
        successful_inner_charge: &InnerChargeLedger,
        successful_outer_charge: &OuterChargeLedger
    ) -> Result<TransactionLedger, ServiceError> {
        Ok(
            self.dao.clone().insert_transaction_ledger(
                InsertableTransactionLedger {
                    transaction_id: registered_transaction.transaction_id,
                    inner_charge_ledger_id: successful_inner_charge.id,
                    outer_charge_ledger_id: successful_outer_charge.id
                }
            ).await?
        )
    }
}