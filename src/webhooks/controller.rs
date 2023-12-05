use actix_web::{
    web,
    get,
    post,
    HttpResponse,
};
use uuid::Uuid;
use crate::api_error::ApiError;
use adyen_webhooks::models::{
    RecurringContractNotificationRequest,
    NotificationResponse
};


#[post("/adyen-webhook/")]
async fn adyen_webhook(notification: web::Json<RecurringContractNotificationRequest>) -> Result<NotificationResponse, ApiError> {

    Ok(
        NotificationResponse {
            notification_response: Some("[accepted]".to_string())
        }
    )

}
