use actix_web::{
    web,
    get,
    post,
    HttpResponse,
};
use uuid::Uuid;
use crate::api_error::ApiError;
use crate::user::dao::{UserDao, UserDaoTrait};
use crate::user::request::CreateUserRequest;
use super::entity::{User, UserMessage};


#[get("/{user_id}/")]
async fn find(user_id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let id = user_id.into_inner();
    info!("Finding user by public id {}", id.clone());
    let user = User::find(&id)?;
    Ok(HttpResponse::Ok().json(user))
}
#[get("/list/")]
async fn list() -> Result<HttpResponse, ApiError> {
    info!("Listing users");
    let users = User::find_all()?;
    info!("Found {} users", users.len());
    Ok(HttpResponse::Ok().json(users))
}

#[post("/")]
async fn create(request: web::Json<CreateUserRequest>) -> Result<HttpResponse, ApiError> {
    info!("Creating user");
    let request = request.into_inner();
    let user = UserDao::new().create(
        &UserMessage {
            email: &request.email,
            password: &request.password
        }
    )?;
    Ok(HttpResponse::Ok().json(user))
}