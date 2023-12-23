use crate::api_error::ApiError;
use reqwest;
use std::future::{Ready, ready};
use std::pin::Pin;
use std::str::FromStr;
use std::task::{Context, Poll};
use actix_web::{dev::ServiceRequest, Error, HttpMessage, HttpResponse};
use actix_web::dev::{Service, ServiceResponse, Transform};
use actix_web::http::header::HeaderMap;
use jsonwebtoken::{Algorithm, decode, DecodingKey, Validation};
use uuid::Uuid;
use crate::auth::constant::JWT_SECRET;
use crate::auth::entity::{Claims, Role};
use crate::auth::jwt::jwt_from_header;
use crate::user::entity::User;



use actix_web::{
    body::EitherBody,
    dev,
    http
};
use futures_util::future::LocalBoxFuture;

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}
pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        // Change this to see the change in outcome in the browser.
        // Usually this boolean would be acquired from a password check or other auth verification.
        let mut is_logged_in = false;

        let uid = match_jwt(request.headers(), &Role::User);
        if let Ok(user_public_id) = uid {
            if let Ok(id) = Uuid::from_str(&user_public_id) {
                if let Ok(user) = User::find(id) {
                    // insert data into extensions if enabled
                    is_logged_in = true;
                    request.extensions_mut()
                        .insert(user);

                    //return Box::pin(self.service.call(request))
                }
            }
        }

        if is_logged_in {
            let res = self.service.call(request);

            Box::pin(async move {
                // forwarded responses map to "left" body
                res.await.map(ServiceResponse::map_into_left_body)
            })
        } else {
            let response = HttpResponse::Unauthorized().finish().map_into_right_body();
            let (request, _pl) = request.into_parts();
            return Box::pin(async { Ok(ServiceResponse::new(request, response)) })
        }
    }
}

fn match_jwt(headers: &HeaderMap, role: &Role) -> Result<String, ApiError> {
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            let decoded = decode::<Claims>(
                &jwt,
                &DecodingKey::from_secret(JWT_SECRET),
                &Validation::new(Algorithm::HS512),
            )
                .map_err(|_| ApiError::new(401, "JWT error".to_string()))?;

            if *role == Role::Admin && Role::from_str(&decoded.claims.role) != Role::Admin {
                return Err(ApiError::new(403, "Permission denied".to_string()));
            }

            Ok(decoded.claims.sub) // this would be user id
        }
        Err(e) => return Err(ApiError::new(401, "Unauthorized".to_string())),
    }

}