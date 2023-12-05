use adyen_webhooks::models::{
    RecurringContractNotificationRequest, 
    RecurringContractNotificationRequestItemWrapper, 
    RecurringContractNotificationRequestItem,
    recurring_contract_notification_request_item::EventCode,
    RecurringContractNotificationAdditionalData
};
pub struct AdyenHandler {}

impl AdyenHandler {
    pub async fn handle(request: NotificationRequest) -> Result<(), ApiError> {
        let notification_items = request.notification_items.to_owned();
        notification_items.iter().for_each(f)

    }

    pub async fn handle_item(item: NotificationRequestItemWrapper) {
        if let Some(inner_item) = item.notification_request_item {
            if inner_item.event_code == EventCode::RecurringContract && inner_item.success == "true" {
                let psp_reference = inner_item.psp_reference;
                let original_psp = inner_item.original_psp;
                if let Some(additional_data) = inner_item.additional_data {
                    if let Some(shopper_reference) = additional_data.shopper_reference {
                        
                    }
                }
            }
        }
    }
}