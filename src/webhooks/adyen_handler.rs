use std::sync::Arc;
use adyen_webhooks::models::{
    RecurringContractNotificationRequest, 
    RecurringContractNotificationRequestItemWrapper, 
    recurring_contract_notification_request_item::EventCode,
};

use crate::wallet::entity::{
    WalletCardAttempt,
    UpdateCardAttempt,
    NewCard,
    Wallet
};

use crate::wallet::constant::WalletCardAttemptStatus;
use crate::wallet::engine::Engine as WalletEngine;
use crate::api_error::ApiError;
use crate::wallet::request::MatchAttemptRequest;

pub struct AdyenHandler {
    wallet_engine: Arc<WalletEngine>
}

impl AdyenHandler {

    pub fn new() -> Self {
        Self {
            wallet_engine: Arc::new(WalletEngine::new())
        }
    }

    pub fn new_with_service(wallet_engine: Arc<WalletEngine>) -> Self {
        Self {
            wallet_engine
        }
    }

    // TODO: need to modify underlying json for client to get all notification types into one
    pub async fn handle(self: Arc<Self>, request: RecurringContractNotificationRequest) -> Result<(), ApiError> {
        if let Some(notification_items) = request.notification_items.to_owned() {
            for item in notification_items.iter() {
                let _ = self.clone().handle_item(item.clone()).await;
            }
        }
        Ok(())
    }

    pub async fn handle_item(self: Arc<Self>, item: RecurringContractNotificationRequestItemWrapper) -> Result<(), ApiError> {
        if let Some(inner_item) = item.notification_request_item {
            if inner_item.event_code == EventCode::RecurringContract && inner_item.success == "true" {
                let psp_reference = inner_item.psp_reference; // new card psp
                let original_psp = inner_item.original_psp; // match psp
                let merchant_reference = inner_item.merchant_reference;
                info!("Match from reference {}: new card {}", merchant_reference, psp_reference);
                let match_attempt_request = MatchAttemptRequest {
                    merchant_reference_id: merchant_reference.clone(),
                    original_psp_reference: original_psp.clone(),
                    psp_reference: psp_reference.clone(),
                };
                let card = self.wallet_engine.clone().attempt_match(
                    &match_attempt_request
                ).await?;
                info!("Added card {} for user id {} with id {}", card.id, card.user_id, psp_reference);
            }
        }
        Ok(())
    }
}