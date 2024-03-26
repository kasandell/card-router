use std::sync::Arc;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use crate::common::model::TransactionMetadata;
use crate::passthrough_card::model::PassthroughCardModel as PassthroughCard;
use super::error::LedgerError;
use crate::ledger::constant::ChargeStatus;
use crate::ledger::dao::{LedgerDao, LedgerDaoTrait};
use crate::ledger::entity::{InnerChargeLedger, InsertableInnerChargeLedger, InsertableOuterChargeLedger, InsertableRegisteredTransaction, InsertableTransactionLedger, OuterChargeLedger, RegisteredTransaction, TransactionLedger};
use crate::ledger::model::{
    InnerChargeLedgerModel,
    OuterChargeLedgerModel,
    RegisteredTransactionModel,
    TransactionLedgerModel
};
use crate::user::model::UserModel as User;
use crate::wallet::model::WalletModel as Wallet;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait LedgerServiceTrait {
    async fn register_transaction_for_user(
        self: Arc<Self>,
        user: &User,
        metadata: &TransactionMetadata,
    ) -> Result<RegisteredTransactionModel, LedgerError>;

    async fn register_failed_inner_charge(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransactionModel,
        metadata: &TransactionMetadata,
        card: &Wallet
    ) -> Result<InnerChargeLedgerModel, LedgerError>;

    async fn register_successful_inner_charge(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransactionModel,
        metadata: &TransactionMetadata,
        card: &Wallet
    ) -> Result<InnerChargeLedgerModel, LedgerError>;

    async fn register_failed_outer_charge(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransactionModel,
        metadata: &TransactionMetadata,
        card: &PassthroughCard
    ) -> Result<OuterChargeLedgerModel, LedgerError>;

    async fn register_successful_outer_charge(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransactionModel,
        metadata: &TransactionMetadata,
        card: &PassthroughCard
    ) -> Result<OuterChargeLedgerModel, LedgerError>;

    async fn register_full_transaction(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransactionModel,
        successful_inner_charge: &InnerChargeLedgerModel,
        successful_outer_charge: &OuterChargeLedgerModel
    ) -> Result<TransactionLedgerModel, LedgerError>;
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
    #[tracing::instrument(skip(self))]
    async fn register_transaction_for_user(
        self: Arc<Self>,
        user: &User,
        metadata: &TransactionMetadata,
    ) -> Result<RegisteredTransactionModel, LedgerError> {
        // TODO: this call takes a long time
        tracing::info!("Registering transaction for user_id={} amount={}", user.id, &metadata.amount_cents);
        let res = self.dao.clone().insert_registered_transaction(
            &InsertableRegisteredTransaction {
                user_id: user.id,
                memo: &metadata.memo,
                amount_cents: metadata.amount_cents,
                mcc: &metadata.mcc
            }
        ).await?.into();
        Ok(res)
    }

    #[tracing::instrument(skip(self))]
    async fn register_failed_inner_charge(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransactionModel,
        metadata: &TransactionMetadata,
        card: &Wallet
    ) -> Result<InnerChargeLedgerModel, LedgerError> {
        tracing::info!("Registering failed inner charge for transaction={} card_id={}", &registered_transaction.transaction_id, &card.id);
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
            ).await?.into()
        )
    }

    #[tracing::instrument(skip(self))]
    async fn register_successful_inner_charge(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransactionModel,
        metadata: &TransactionMetadata,
        card: &Wallet
    ) -> Result<InnerChargeLedgerModel, LedgerError> {
        tracing::info!("Registering successful inner charge for transaction={} card_id={}", &registered_transaction.transaction_id, &card.id);
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
            ).await?.into()
        )
    }

    #[tracing::instrument(skip(self))]
    async fn register_failed_outer_charge(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransactionModel,
        metadata: &TransactionMetadata,
        card: &PassthroughCard
    ) -> Result<OuterChargeLedgerModel, LedgerError> {
        // TODO: do some assertions that everything is associated
        tracing::info!("Registering failed outer charge for transaction={} passthrough_card_id={}", &registered_transaction.transaction_id, &card.id);
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
            ).await?.into()
        )
    }

    #[tracing::instrument(skip(self))]
    async fn register_successful_outer_charge(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransactionModel,
        metadata: &TransactionMetadata,
        card: &PassthroughCard
    ) -> Result<OuterChargeLedgerModel, LedgerError> {
        // TODO: do some assertions that everything is associated
        tracing::info!("Registering succesful outer charge for transaction={} passthrough_card_id={}", &registered_transaction.transaction_id, &card.id);
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
            ).await?.into()
        )
    }

    #[tracing::instrument(skip(self))]
    async fn register_full_transaction(
        self: Arc<Self>,
        registered_transaction: &RegisteredTransactionModel,
        successful_inner_charge: &InnerChargeLedgerModel,
        successful_outer_charge: &OuterChargeLedgerModel
    ) -> Result<TransactionLedgerModel, LedgerError> {
        tracing::info!("Registering full transaction for transaction_id={} inner_id={} outer_id={}", &registered_transaction.transaction_id, successful_inner_charge.id, successful_outer_charge.id);
        Ok(
            self.dao.clone().insert_transaction_ledger(
                &InsertableTransactionLedger {
                    registered_transaction_id: registered_transaction.id,
                    inner_charge_ledger_id: successful_inner_charge.id,
                    outer_charge_ledger_id: successful_outer_charge.id
                }
            ).await?.into()
        )
    }
}