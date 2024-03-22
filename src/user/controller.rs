use actix_web::{web, get, post, HttpResponse, services};
use crate::auth::entity::Claims;
use crate::middleware::services::Services;
use crate::user::request::CreateUserRequest;
use crate::user::response::UserResponse;
use crate::user::service::UserServiceTrait;
use super::error::UserError;

#[post("/")]
async fn create(
    request: web::Json<CreateUserRequest>,
    claims: Claims,
    services: web::Data<Services>
) -> Result<HttpResponse, UserError> {
    tracing::info!("Get or Creating user");
    let Some(auth0) = claims.sub else {return Err(
        UserError::Unauthorized("unauthorized".into())
    );};
    let request = request.into_inner();
    let user = services.user_service.clone().get_or_create(
        &auth0,
        &request.email
    ).await?;
    Ok(HttpResponse::Ok().json(
        UserResponse::from(&user)
    ))
}