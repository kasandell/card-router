use actix_web::{put, post, get, HttpResponse};
use crate::api_error::ApiError;

#[post("/create-card/")]
async fn create_card() -> Result<HttpResponse, ApiError> {
    Ok(
        HttpResponse::Ok().finish()
    )
}

#[put("/pause-card/")]
async fn pause_card() -> Result<HttpResponse, ApiError> {
    Ok(
        HttpResponse::Ok().finish()
    )
}

#[put("/unpause-card/")]
async fn unpause_card() -> Result<HttpResponse, ApiError> {
    Ok(
        HttpResponse::Ok().finish()
    )
}

#[put("/cancel-card/")]
async fn cancel_card() -> Result<HttpResponse, ApiError> {
    Ok(
        HttpResponse::Ok().finish()
    )
}

