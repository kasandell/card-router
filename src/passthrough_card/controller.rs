use actix_web::{put, post, get, HttpResponse, web};
use crate::api_error::ApiError;
use crate::passthrough_card::response::HasActiveResponse;
use crate::user::entity::User;
use super::engine::Engine;

#[post("/create-card/")]
async fn create_card(
    user: web::ReqData<User>
) -> Result<HttpResponse, ApiError> {
    Ok(
        HttpResponse::Ok().finish()
    )
}

#[get("/active-card/")]
async fn active_card(
    user: web::ReqData<User>
) -> Result<HttpResponse, ApiError> {
    let user = user.into_inner();
    let engine = Engine::new();
    let has_active = engine.user_has_active_card(&user)?;
    Ok(
        HttpResponse::Ok().json(
            HasActiveResponse {
                has_active
            }
        )
    )
}

#[put("/pause-card/")]
async fn pause_card(
    user: web::ReqData<User>
) -> Result<HttpResponse, ApiError> {
    Ok(
        HttpResponse::Ok().finish()
    )
}

#[put("/unpause-card/")]
async fn unpause_card(
    user: web::ReqData<User>
) -> Result<HttpResponse, ApiError> {
    Ok(
        HttpResponse::Ok().finish()
    )
}

#[put("/cancel-card/")]
async fn cancel_card(
    user: web::ReqData<User>
) -> Result<HttpResponse, ApiError> {
    Ok(
        HttpResponse::Ok().finish()
    )
}

