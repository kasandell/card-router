use actix_web::{
    web,
    post,
    HttpResponse,
};
use crate::error::error::ServiceError;
use crate::asa::request::AsaRequest;
use crate::middleware::services::Services;

#[post("/lithic-asa-webhook/")]
async fn lithic_asa_webhook(
    asa: web::Json<AsaRequest>,
    services: web::Data<Services>
) -> Result<HttpResponse, ServiceError> {
    let resp = services.lithic_handler.clone().handle(asa.into_inner()).await?;
    Ok(
        HttpResponse::Ok().json(
            resp
        )
    )
}