use std::sync::{Arc, Mutex};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;

use uuid::Uuid;
use crate::passthrough_card::entity::PassthroughCard;
use super::error::LedgerError;
use crate::ledger::constant::ChargeStatus;
use crate::ledger::dao::{LedgerDao, LedgerDaoTrait};
use crate::ledger::entity::{InnerChargeLedger, InsertableInnerChargeLedger, InsertableOuterChargeLedger, InsertableRegisteredTransaction, InsertableTransactionLedger, OuterChargeLedger, RegisteredTransaction, TransactionLedger, TransactionMetadata};
use crate::user::entity::User;
use crate::wallet::entity::Wallet;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait LedgerServiceTrait {
    async fn register_transaction_for_user(
        self: Arc<Self>,
        user: &User,
        metadata: &TransactionMetadata,
    ) -> Result<RegisteredTransaction, LedgerError>;

    async fn register_failed_inner_charge(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransaction,
        metadata: &TransactionMetadata,
        card: &Wallet
    ) -> Result<InnerChargeLedger, LedgerError>;

    async fn register_successful_inner_charge(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransaction,
        metadata: &TransactionMetadata,
        card: &Wallet
    ) -> Result<InnerChargeLedger, LedgerError>;

    async fn register_failed_outer_charge(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransaction,
        metadata: &TransactionMetadata,
        card: &PassthroughCard
    ) -> Result<OuterChargeLedger, LedgerError>;

    async fn register_successful_outer_charge(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransaction,
        metadata: &TransactionMetadata,
        card: &PassthroughCard
    ) -> Result<OuterChargeLedger, LedgerError>;

    async fn register_full_transaction(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransaction,
        successful_inner_charge: &InnerChargeLedger,
        successful_outer_charge: &OuterChargeLedger
    ) -> Result<TransactionLedger, LedgerError>;
}

pub struct LedgerService {
    dao: Arc<dyn LedgerDaoTrait>
}

impl LedgerService {
    pub fn new() -> Self {
        Self {
            dao: Arc::new(LedgerDao {})
        }
    }
}

#[async_trait(?Send)]
impl LedgerServiceTrait for LedgerService {
    async fn register_transaction_for_user(
        self: Arc<Self>,
        user: &User,
        metadata: &TransactionMetadata,
    ) -> Result<RegisteredTransaction, LedgerError> {
        // TODO: this call takes a long time
        let res = self.dao.clone().insert_registered_transaction(
            &InsertableRegisteredTransaction {
                user_id: user.id,
                //transaction_id: Uuid::new_v4(),
                memo: &metadata.memo,
                amount_cents: metadata.amount_cents,
                mcc: &metadata.mcc
            }
        ).await?;
        Ok(res)
    }

    async fn register_failed_inner_charge(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransaction,
        metadata: &TransactionMetadata,
        card: &Wallet
    ) -> Result<InnerChargeLedger, LedgerError> {
        // TODO: should do some verification somewhere that cards are associated with the correct user for the outer txn
        Ok(
            self.dao.clone().insert_inner_charge(
                &InsertableInnerChargeLedger {
                    registered_transaction_id: registered_transaction.id,
                    user_id: registered_transaction.user_id,
                    wallet_card_id: card.id,
                    amount_cents: metadata.amount_cents,
                    status: ChargeStatus::Fail,
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
    ) -> Result<InnerChargeLedger, LedgerError> {
        // TODO: should do some verification somewhere that cards are associated with the correct user for the outer txn
        Ok(
            self.dao.clone().insert_inner_charge(
                &InsertableInnerChargeLedger {
                    registered_transaction_id: registered_transaction.id,
                    user_id: registered_transaction.user_id,
                    wallet_card_id: card.id,
                    amount_cents: metadata.amount_cents,
                    status: ChargeStatus::Success,
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
    ) -> Result<OuterChargeLedger, LedgerError> {
        // TODO: do some assertions that everything is associated
        Ok(
            self.dao.clone().insert_outer_charge(
                &InsertableOuterChargeLedger {
                    registered_transaction_id: registered_transaction.id,
                    user_id: registered_transaction.user_id,
                    passthrough_card_id: card.id,
                    amount_cents: metadata.amount_cents,
                    status: ChargeStatus::Fail,
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
    ) -> Result<OuterChargeLedger, LedgerError> {
        // TODO: do some assertions that everything is associated
        Ok(
            self.dao.clone().insert_outer_charge(
                &InsertableOuterChargeLedger {
                    registered_transaction_id: registered_transaction.id,
                    user_id: registered_transaction.user_id,
                    passthrough_card_id: card.id,
                    amount_cents: metadata.amount_cents,
                    status: ChargeStatus::Success,
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
    ) -> Result<TransactionLedger, LedgerError> {
        Ok(
            self.dao.clone().insert_transaction_ledger(
                &InsertableTransactionLedger {
                    registered_transaction_id: registered_transaction.id,
                    inner_charge_ledger_id: successful_inner_charge.id,
                    outer_charge_ledger_id: successful_outer_charge.id
                }
            ).await?
        )
    }
}