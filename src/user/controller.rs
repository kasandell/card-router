use actix_web::{web, get, post, HttpResponse, services};
use uuid::Uuid;
use crate::api_error::ApiError;
use crate::middleware::claims::Claims;
use crate::middleware::services::Services;
use crate::user::dao::{UserDao, UserDaoTrait};
use crate::user::request::CreateUserRequest;
use crate::user::service::UserServiceTrait;
use super::entity::{User, UserMessage};


#[get("/{user_id}/")]
async fn find(
    user_id: web::Path<Uuid>,
    services: web::Data<Services>
) -> Result<HttpResponse, ApiError> {
    let id = user_id.into_inner();
    info!("Finding user by public id {}", id.clone());
    let user = services.user_dao.clone().find(&id).await?;
    Ok(HttpResponse::Ok().json(user))
}
#[get("/list/")]
async fn list(
    services: web::Data<Services>
) -> Result<HttpResponse, ApiError> {
    info!("Listing users");
    let users = services.user_dao.clone().find_all().await?;
    info!("Found {} users", users.len());
    Ok(HttpResponse::Ok().json(users))
}

#[post("/")]
async fn create(
    request: web::Json<CreateUserRequest>,
    claims: Claims,
    services: web::Data<Services>
) -> Result<HttpResponse, ApiError> {
    info!("Get or Creating user");
    let Some(auth0) = claims.sub else {return Err(ApiError::new(401, "unauthorized".to_string()));};
    let request = request.into_inner();
    let user = services.user_service.get_or_create(
        &auth0,
        &request.email
    ).await?;
    /*
    let user = services.user_dao.clone().create(
        &UserMessage {
            email: &request.email,
            auth0_user_id: &auth0
        }
    ).await?;
     */
    Ok(HttpResponse::Ok().json(user))
}