use reqwest;
use std::future::{Ready, ready};
use std::sync::Arc;
use actix_web::{dev::ServiceRequest, Error, FromRequest, HttpMessage, HttpResponse};
use actix_web::dev::{Service, ServiceResponse, Transform};
use crate::user::entity::User;



use actix_web::{
    body::EitherBody,
    dev,
    http
};
use futures_util::future::LocalBoxFuture;
use crate::auth::entity::Claims;

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

    #[tracing::instrument(skip(self))]
    fn call(&self, mut request: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            // TODO: swap this to a call to auth0
            let mut is_logged_in = false;
            let (req, payload)= request.parts_mut();
            let claims = Claims::from_request(req, payload).await?;
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
        })
    }
}