use actix_web::{
    web,
    post,
    HttpResponse,
};

use crate::api_error::ApiError;
use adyen_webhooks::models::{
    RecurringContractNotificationRequest,
    NotificationResponse
};
use crate::webhooks::adyen_handler::AdyenHandler;


#[post("/adyen-webhook/")]
async fn adyen_webhook(notification: web::Json<RecurringContractNotificationRequest>) -> Result<HttpResponse, ApiError> {
    AdyenHandler::handle(notification.into_inner()).await?;
    Ok(
        HttpResponse::Ok().json(
            NotificationResponse {
                notification_response: Some("[accepted]".to_string())
            }
        )
    )

}
