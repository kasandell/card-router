use actix_web::{
    web,
    post,
    HttpResponse,
};
use actix_web::web::Bytes;

use crate::api_error::ApiError;
use crate::asa::request::AsaRequest;
use adyen_webhooks::models::{
    RecurringContractNotificationRequest,
    NotificationResponse
};
use crate::webhooks::adyen_handler::AdyenHandler;
use crate::webhooks::lithic_handler::LithicHandler;


#[post("/adyen-webhook/")]
async fn adyen_webhook(notification: web::Json<RecurringContractNotificationRequest>) -> Result<HttpResponse, ApiError> {
    let handler = AdyenHandler::new();
    handler.handle(notification.into_inner()).await?;
    Ok(
        HttpResponse::Ok().json(
            NotificationResponse {
                notification_response: Some("[accepted]".to_string())
            }
        )
    )
}

#[post("/lithic-asa-webhook/")]
async fn lithic_asa_webhook(asa: web::Json<AsaRequest>) -> Result<HttpResponse, ApiError> {
    let handler = LithicHandler::new();
    Ok(
        HttpResponse::Ok().json(
            handler.handle(asa.into_inner()).await?
        )
    )
}


/*
#[post("/lithic-asa-webhook/")]
async fn lithic_asa_webhook(asa: Bytes) -> Result<HttpResponse, ApiError> {
    match String::from_utf8(asa.to_vec()) {
        Ok(text) => {
            println!("{}", text);
        }
        Err(_) => {
            println!("COULD NOT DESERIALIZE");
        }
    }

    Ok(
        HttpResponse::Ok().finish()
    )
}
 */
