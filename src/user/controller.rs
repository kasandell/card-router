use actix_web::{web, get, post, HttpResponse, services};
use uuid::Uuid;
use crate::api_error::ApiError;
use crate::middleware::services::Services;
use crate::user::dao::{UserDao, UserDaoTrait};
use crate::user::request::CreateUserRequest;
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
    services: web::Data<Services>
) -> Result<HttpResponse, ApiError> {
    info!("Creating user");
    let request = request.into_inner();
    let user = services.user_dao.clone().create(
        &UserMessage {
            email: &request.email,
            password: &request.password
        }
    ).await?;
    Ok(HttpResponse::Ok().json(user))
}