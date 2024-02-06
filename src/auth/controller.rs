use actix_web::{HttpResponse, post, web};
use crate::api_error::ApiError;
use crate::auth::entity::Role;
use crate::auth::jwt::create_jwt;
use crate::auth::request::LoginRequest;
use crate::auth::response::LoginResponse;
use crate::user::entity::User;

#[post("/login/")]
async fn login(request: web::Json<LoginRequest>) -> Result<HttpResponse, ApiError>{
    let request = request.into_inner();
    let user = User::find_by_email_password(
        request.email.clone(),
        request.password.clone()
    );
    match user {
        Ok(user) => {
            let token = create_jwt(&user.public_id.to_string(), &Role::User)
                .map_err(|e| ApiError::new(401, "Invalid credentials".to_string()))?;
            return Ok(
                HttpResponse::Ok().json(
                    &LoginResponse { token }
                )
            );
        },
        Err(err) => {
            return Err(ApiError::new(401, "Invalid user".to_string()));
        }
    }
    Err(ApiError::new(500, "Not implemented".to_string()))
}

#[post("/logout/")]
async fn logout() -> Result<HttpResponse, ApiError>{
    Err(ApiError::new(500, "Not implemented".to_string()))
}