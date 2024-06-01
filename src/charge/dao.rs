use std::sync::Arc;
use uuid::Uuid;
use crate::error::data_error::DataError;
use crate::charge::entity::{WalletCardCharge, InsertableWalletCardCharge, InsertablePassthroughCardCharge, InsertableRegisteredTransaction, InsertableSuccessfulEndToEndCharge, PassthroughCardCharge, RegisteredTransaction, SuccessfulEndToEndCharge, InsertableExpectedWalletChargeReference, ExpectedWalletChargeReference};
use async_trait::async_trait;

#[cfg(test)]
use mockall::{automock, predicate::*};
use crate::util::transaction::Transaction;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ChargeDaoTrait {
    async fn insert_registered_transaction<'a>(self: Arc<Self>, database_transaction: &mut Transaction<'_, '_>, registered_transaction: &InsertableRegisteredTransaction<'a>) -> Result<RegisteredTransaction, DataError>;
    async fn get_registered_transaction_by_transaction_id(self: Arc<Self>, id: &Uuid) -> Result<RegisteredTransaction, DataError>;
    async fn get_registered_transaction(self: Arc<Self>, id: i32) -> Result<RegisteredTransaction, DataError>;

    async fn insert_expected_wallet_charge_reference<'a>(self: Arc<Self>, transaction: &mut Transaction<'_, '_>,reference: &InsertableExpectedWalletChargeReference) -> Result<ExpectedWalletChargeReference, DataError>;

    async fn insert_wallet_charge<'a>(self: Arc<Self>, transaction: &mut Transaction<'_, '_>, charge: &InsertableWalletCardCharge) -> Result<WalletCardCharge, DataError>;
    async fn get_wallet_charges_by_registered_transaction(self: Arc<Self>, registered_transaction: i32) -> Result<Vec<WalletCardCharge>, DataError>;
    async fn get_successful_wallet_charge_by_registered_transaction(self: Arc<Self>, registered_transaction: i32) -> Result<WalletCardCharge, DataError>;
    async fn get_wallet_charge_by_id(self: Arc<Self>, id: i32) -> Result<WalletCardCharge, DataError>;

    async fn insert_passthrough_card_charge<'a>(self: Arc<Self>, transaction: &mut Transaction<'_, '_>, charge: &InsertablePassthroughCardCharge) -> Result<PassthroughCardCharge, DataError>;
    async fn get_passthrough_card_charge_by_registered_transaction(self: Arc<Self>, registered_transaction: i32) -> Result<PassthroughCardCharge, DataError>;
    async fn get_passthrough_card_charge_by_id(self: Arc<Self>, id: i32) -> Result<PassthroughCardCharge, DataError>;

    async fn insert_successful_end_to_end_charge<'a>(self: Arc<Self>, transaction: &mut Transaction<'_, '_>, charge: &InsertableSuccessfulEndToEndCharge) -> Result<SuccessfulEndToEndCharge, DataError>;
    async fn get_successful_end_to_end_charge_by_registered_transaction_id(self: Arc<Self>, id: i32) -> Result<SuccessfulEndToEndCharge, DataError>;
    async fn get_successful_end_to_end_charge_by_id(self: Arc<Self>, id: i32) -> Result<SuccessfulEndToEndCharge, DataError>;
}

pub struct ChargeDao {}

impl ChargeDao {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl ChargeDaoTrait for ChargeDao {
    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn insert_registered_transaction<'a>(self: Arc<Self>, database_transaction: &mut Transaction<'_, '_>, registered_transaction: &InsertableRegisteredTransaction<'a>) -> Result<RegisteredTransaction, DataError> {
        RegisteredTransaction::insert(database_transaction, registered_transaction).await
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn get_registered_transaction_by_transaction_id(self: Arc<Self>, id: &Uuid) -> Result<RegisteredTransaction, DataError> {
        RegisteredTransaction::get_by_transaction_id(id).await
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn get_registered_transaction(self: Arc<Self>, id: i32) -> Result<RegisteredTransaction, DataError> {
        RegisteredTransaction::get(id).await
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn insert_expected_wallet_charge_reference<'a>(self: Arc<Self>, transaction: &mut Transaction<'_, '_>,reference: &InsertableExpectedWalletChargeReference) -> Result<ExpectedWalletChargeReference, DataError> {
        ExpectedWalletChargeReference::insert(
            transaction,
            reference
        ).await
    }
    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn insert_wallet_charge<'a>(self: Arc<Self>, transaction: &mut Transaction<'_, '_>, charge: &InsertableWalletCardCharge) -> Result<WalletCardCharge, DataError> {
        WalletCardCharge::insert(transaction, charge).await
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn get_wallet_charges_by_registered_transaction(self: Arc<Self>, registered_transaction: i32) -> Result<Vec<WalletCardCharge>, DataError> {
        WalletCardCharge::get_wallet_card_charges_by_registered_transaction(registered_transaction).await
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn get_successful_wallet_charge_by_registered_transaction(self: Arc<Self>, registered_transaction: i32) -> Result<WalletCardCharge, DataError> {
        WalletCardCharge::get_successful_wallet_card_charge_by_registered_transaction(registered_transaction).await
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn get_wallet_charge_by_id(self: Arc<Self>, id: i32) -> Result<WalletCardCharge, DataError> {
        WalletCardCharge::get_by_id(id).await
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn insert_passthrough_card_charge<'a>(self: Arc<Self>, transaction: &mut Transaction<'_, '_>, charge: &InsertablePassthroughCardCharge) -> Result<PassthroughCardCharge, DataError> {
        PassthroughCardCharge::insert(transaction, charge).await
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn get_passthrough_card_charge_by_registered_transaction(self: Arc<Self>, registered_transaction: i32) -> Result<PassthroughCardCharge, DataError> {
        PassthroughCardCharge::get_outer_charge_by_registered_transaction(registered_transaction).await
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn get_passthrough_card_charge_by_id(self: Arc<Self>, id: i32) -> Result<PassthroughCardCharge, DataError> {
        PassthroughCardCharge::get_by_id(id).await
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn insert_successful_end_to_end_charge<'a>(self: Arc<Self>, transaction: &mut Transaction<'_, '_>, charge: &InsertableSuccessfulEndToEndCharge) -> Result<SuccessfulEndToEndCharge, DataError> {
        SuccessfulEndToEndCharge::insert(transaction, charge).await
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn get_successful_end_to_end_charge_by_registered_transaction_id(self: Arc<Self>, id: i32) -> Result<SuccessfulEndToEndCharge, DataError> {
        SuccessfulEndToEndCharge::get_by_registered_transaction_id(id).await
    }

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip(self)))]
    async fn get_successful_end_to_end_charge_by_id(self: Arc<Self>, id: i32) -> Result<SuccessfulEndToEndCharge, DataError> {
        SuccessfulEndToEndCharge::get_by_id(id).await
    }



}