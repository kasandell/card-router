use actix_web::{
    web,
    get,
    put,
    post,
    delete,
    HttpResponse,
    Responder
};
use serde_json::json;
use uuid::Uuid;

use crate::api_error::ApiError;
use super::entity::{User, UserMessage};


#[get("/{user_id}/")]
async fn find(user_id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let id = user_id.into_inner();
    info!("Finding user by public id {}", id.clone());
    let user = User::find(id.clone())?;
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
async fn create(user: web::Json<UserMessage>) -> Result<HttpResponse, ApiError> {
    info!("Creating user");
    let user = User::create(user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}