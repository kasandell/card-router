use std::time::Instant;
use actix_web::{
    web,
    post,
    HttpResponse,
};
use crate::error::api_error::ApiError;
use crate::asa::request::AsaRequest;
use adyen_webhooks::models::{
    RecurringContractNotificationRequest,
    NotificationResponse,
    AuthorisationNotificationRequest
};
use crate::webhooks::adyen_handler::AdyenHandler;
use crate::webhooks::lithic_handler::LithicHandler;
use crate::middleware::services::Services;


// TODO: create a special error here that always unwraps to 200
#[post("/adyen-webhook/")]
async fn adyen_webhook(
    notification: web::Json<AuthorisationNotificationRequest>,
    services: web::Data<Services>
) -> Result<HttpResponse, ApiError> {
    //services.adyen_handler.clone().handle(notification.into_inner()).await?;
    Ok(
        HttpResponse::Ok().json(
            NotificationResponse {
                notification_response: Some("[accepted]".to_string())
            }
        )
    )
}

#[post("/lithic-asa-webhook/")]
async fn lithic_asa_webhook(
    asa: web::Json<AsaRequest>,
    services: web::Data<Services>
) -> Result<HttpResponse, ApiError> {
    let mut start = Instant::now();
    let resp = services.lithic_handler.clone().handle(asa.into_inner()).await?;
    println!("Lithic handler took {:?}", start.elapsed());
    Ok(
        HttpResponse::Ok().json(
            resp
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
