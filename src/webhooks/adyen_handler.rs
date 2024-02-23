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
    wallet_engine: Box<WalletEngine>
}

impl AdyenHandler {

    pub fn new() -> Self {
        Self {
            wallet_engine: Box::new(WalletEngine::new())
        }
    }

    #[cfg(test)]
    pub fn new_with_service(wallet_engine: Box<WalletEngine>) -> Self {
        Self {
            wallet_engine
        }
    }

    pub async fn handle(&self, request: RecurringContractNotificationRequest) -> Result<(), ApiError> {
        if let Some(notification_items) = request.notification_items.to_owned() {
            notification_items.iter().for_each(|item| {
                let _ = self.handle_item(item.clone());
                return ().into();
            });
        }
        Ok(())
    }

    pub fn handle_item(&self, item: RecurringContractNotificationRequestItemWrapper) -> Result<(), ApiError> {
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
                let card = self.wallet_engine.attempt_match(
                    &match_attempt_request
                )?;
                info!("Added card {} for user id {} with id {}", card.id, card.user_id, psp_reference);
            }
        }
        Ok(())
    }
}