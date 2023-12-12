use adyen_webhooks::models::{
    RecurringContractNotificationRequest, 
    RecurringContractNotificationRequestItemWrapper, 
    RecurringContractNotificationRequestItem,
    recurring_contract_notification_request_item::EventCode,
    RecurringContractNotificationAdditionalData
};
use diesel::helper_types::Update;

use crate::wallet::entity::{
    WalletCardAttempt,
    InsertableCardAttempt,
    UpdateCardAttempt,
    NewCard,
    Wallet
};

use crate::wallet::constant::WalletCardAttemptStatus;
use crate::api_error::ApiError;

pub struct AdyenHandler {}

impl AdyenHandler {
    pub async fn handle(request: RecurringContractNotificationRequest) -> Result<(), ApiError> {
        if let Some(notification_items) = request.notification_items.to_owned() {
            notification_items.iter().for_each(|item| {
                AdyenHandler::handle_item(item.clone());
                return ().into();
            });
        }
        Ok(())
    }

    pub fn handle_item(item: RecurringContractNotificationRequestItemWrapper) -> Result<(), ApiError> {
        if let Some(inner_item) = item.notification_request_item {
            if inner_item.event_code == EventCode::RecurringContract && inner_item.success == "true" {
                let psp_reference = inner_item.psp_reference; // new card psp
                let original_psp = inner_item.original_psp; // match psp
                let merchant_reference = inner_item.merchant_reference;
                info!("Match from reference {}: new card {}", merchant_reference, psp_reference);
                let card = WalletCardAttempt::find_by_reference_id(merchant_reference)?;
                info!("Found wallet card attempt id {}", card.id);
                let ret = WalletCardAttempt::update_card(card.id, UpdateCardAttempt {
                    recurring_detail_reference: psp_reference.clone(), 
                    psp_id: original_psp,
                    status: WalletCardAttemptStatus::MATCHED.as_str()
                })?;
                info!("Updated to matched");
                let user_card = Wallet::insert_card(
                    NewCard {
                        user_id: card.user_id,
                        payment_method_id: psp_reference.clone(),
                        credit_card_id: card.credit_card_id,
                        wallet_card_attempt_id: card.id
                    }
                )?;
                info!("Added card {} for user id {} with id {}", user_card.id, user_card.user_id, psp_reference);
            }
        }
        Ok(())
    }
}