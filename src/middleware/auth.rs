use crate::api_error::ApiError;
use reqwest;
use std::future::{Ready, ready};
use std::str::FromStr;
use std::sync::Arc;
use actix_web::{dev::ServiceRequest, Error, FromRequest, HttpMessage, HttpResponse};
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

impl<S: 'static, B> Transform<S, ServiceRequest> for Auth
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
        ready(Ok(AuthMiddleware {
            service: Arc::new(service),
        }))
    }
}

pub struct AuthMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: Arc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, mut request: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            // TODO: swap this to a call to auth0
            let mut is_logged_in = false;
            let uid = match_jwt(request.headers(), &Role::User);
            let (req, payload)= request.parts_mut();
            let claims = crate::middleware::claims::Claims::from_request(req, payload).await?;
            if let Some(auth0_id) = claims.sub {
                if let Ok(user) = User::find_by_auth0_id(&auth0_id).await {
                    // insert data into extensions if enabled
                    is_logged_in = true;
                    request.extensions_mut()
                        .insert(user);
                }
            }

            if is_logged_in {
                let res = svc.call(request);
                // forwarded responses map to "left" body
                return res.await.map(ServiceResponse::map_into_left_body)
            } else {
                let response = HttpResponse::Unauthorized().finish().map_into_right_body();
                let (request, _pl) = request.into_parts();
                return Ok(ServiceResponse::new(request, response))
            }
            /*
            Could be useful later
            let mut body = BytesMut::new();
            let mut stream = req.take_payload();
            while let Some(chunk) = stream.next().await {
                body.extend_from_slice(&chunk?);
            }

            println!("request body: {body:?}");
            let res = svc.call(req).await?;

            println!("response: {:?}", res.headers());

            Ok(res)
             */
        })
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