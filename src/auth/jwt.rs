use actix_web::http::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use crate::api_error::ApiError;
use crate::auth::entity::{Claims, Role};
use super::constant::{BEARER, JWT_SECRET};

pub fn create_jwt(uid: &str, role: &Role) -> Result<String, ApiError> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: uid.to_owned(),
        role: role.to_string(),
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| ApiError::new(401, "Unauthorized".to_string()))
}

pub fn jwt_from_header(headers: &HeaderMap) -> Result<String, ApiError> {
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(ApiError::new(401, "No auth header".to_string())),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Err(ApiError::new(401, "No auth header".to_string())),
    };
    if !auth_header.starts_with(BEARER) {
        return Err(ApiError::new(401, "Invalid auth header".to_string()));
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}
