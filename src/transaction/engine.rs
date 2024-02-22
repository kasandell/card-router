use uuid::Uuid;
use crate::passthrough_card::entity::PassthroughCard;
use crate::service_error::ServiceError;
use crate::transaction::constant::ChargeStatus;
use crate::transaction::entity::{InnerChargeLedger, InsertableInnerChargeLedger, InsertableOuterChargeLedger, InsertableRegisteredTransaction, InsertableTransactionLedger, OuterChargeLedger, RegisteredTransaction, TransactionLedger, TransactionMetadata};
use crate::user::entity::User;
use crate::wallet::entity::Wallet;

#[derive(Clone, Debug)]
pub struct Engine {}

impl Engine {
    pub fn register_transaction_for_user(
        user: &User,
        metadata: &TransactionMetadata,
    ) -> Result<RegisteredTransaction, ServiceError> {
        Ok(
            RegisteredTransaction::insert(
            InsertableRegisteredTransaction {
                    user_id: user.id,
                    transaction_id: Uuid::new_v4(),
                    memo: metadata.memo.to_string(),
                    amount_cents: metadata.amount_cents,
                    mcc: metadata.mcc.to_string()
                }
            )?
        )
    }

    pub fn register_failed_inner_charge(
        registered_transaction: &RegisteredTransaction,
        metadata: &TransactionMetadata,
        card: &Wallet
    ) -> Result<InnerChargeLedger, ServiceError> {
        // TODO: should do some verification somewhere that cards are associated with the correct user for the outer txn
        Ok(
            InnerChargeLedger::insert(
                InsertableInnerChargeLedger {
                    registered_transaction_id: registered_transaction.transaction_id,
                    user_id: registered_transaction.user_id,
                    wallet_card_id: card.id,
                    amount_cents: metadata.amount_cents,
                    status: ChargeStatus::Fail.as_str(),
                    is_success: None,
                }
            )?
        )
    }

    pub fn register_successful_inner_charge(
        registered_transaction: &RegisteredTransaction,
        metadata: &TransactionMetadata,
        card: &Wallet
    ) -> Result<InnerChargeLedger, ServiceError> {
        // TODO: should do some verification somewhere that cards are associated with the correct user for the outer txn
        Ok(
            InnerChargeLedger::insert(
                InsertableInnerChargeLedger {
                    registered_transaction_id: registered_transaction.transaction_id,
                    user_id: registered_transaction.user_id,
                    wallet_card_id: card.id,
                    amount_cents: metadata.amount_cents,
                    status: ChargeStatus::Success.as_str(),
                    is_success: Some(true),
                }
            )?
        )
    }

    pub fn register_failed_outer_charge(
        registered_transaction: &RegisteredTransaction,
        metadata: &TransactionMetadata,
        card: &PassthroughCard
    ) -> Result<OuterChargeLedger, ServiceError> {
        // TODO: do some assertions that everything is associated
        Ok(
            OuterChargeLedger::insert(
                InsertableOuterChargeLedger {
                    registered_transaction_id: registered_transaction.transaction_id,
                    user_id: registered_transaction.user_id,
                    passthrough_card_id: card.id,
                    amount_cents: metadata.amount_cents,
                    status: ChargeStatus::Fail.as_str(),
                    is_success: None,
                }
            )?
        )
    }

    pub fn register_successful_outer_charge(
        registered_transaction: &RegisteredTransaction,
        metadata: &TransactionMetadata,
        card: &PassthroughCard
    ) -> Result<OuterChargeLedger, ServiceError> {
        // TODO: do some assertions that everything is associated
        Ok(
            OuterChargeLedger::insert(
                InsertableOuterChargeLedger {
                    registered_transaction_id: registered_transaction.transaction_id,
                    user_id: registered_transaction.user_id,
                    passthrough_card_id: card.id,
                    amount_cents: metadata.amount_cents,
                    status: ChargeStatus::Success.as_str(),
                    is_success: Some(true),
                }
            )?
        )
    }

    pub fn register_full_transaction(
        registered_transaction: &RegisteredTransaction,
        successful_inner_charge: &InnerChargeLedger,
        successful_outer_charge: &OuterChargeLedger
    ) -> Result<TransactionLedger, ServiceError> {
        Ok(
            TransactionLedger::insert(
                InsertableTransactionLedger {
                    transaction_id: registered_transaction.transaction_id,
                    inner_charge_ledger_id: successful_inner_charge.id,
                    outer_charge_ledger_id: successful_outer_charge.id
                }
            )?
        )
    }
}