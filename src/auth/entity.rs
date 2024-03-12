use actix_web::{
    http::Uri,
    Error, FromRequest,
};
use awc::Client;
use actix_web_httpauth::extractors::bearer::BearerAuth;

use jsonwebtoken::{
    decode, decode_header,
    jwk::{AlgorithmParameters, JwkSet},
    Algorithm, DecodingKey, Validation,
};
use serde::Deserialize;
use std::{collections::HashSet, future::Future, pin::Pin, env};
use futures_util::TryStreamExt;
use crate::error::api_error::ApiError;
use crate::constant;
use crate::error::error_type::ErrorType;

#[derive(Clone, Deserialize)]
pub struct Auth0Config {
    audience: String,
    domain: String,
}

impl Default for Auth0Config {
    fn default() -> Self {
        Self {
            audience: env::var(constant::env_key::AUTH0_AUDIENCE).expect("should exist"),
            domain: env::var(constant::env_key::AUTH0_DOMAIN).expect("should exist"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Claims {
    pub permissions: Option<HashSet<String>>,
    pub sub: Option<String>,
    pub role: Option<String>,
    pub exp: Option<usize>,
    pub scope: Option<String>,
}

impl Claims {
    pub fn validate_permissions(&self, required_permissions: &HashSet<String>) -> bool {
        self.permissions.as_ref().map_or(false, |permissions| permissions.is_superset(required_permissions))
    }
}

impl FromRequest for Claims {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    //type Config = ();

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let Some(config) = req.app_data::<Auth0Config>().cloned() else {
            return Box::pin(async move {
                return Err(ApiError::new(ErrorType::InternalServerError, "Can't get config").into());
            })
        };
        let extractor = BearerAuth::extract(req);
        Box::pin(async move {
            let credentials = extractor.await.map_err(
                |_| ApiError::new(ErrorType::Unauthorized, "Can't get credentials")
            )?;
            let token = credentials.token();
            let header = decode_header(token).map_err(|_| ApiError::new(ErrorType::Unauthorized, "Can't decode"))?;
            let kid = header.kid.ok_or_else(|| ApiError::new(ErrorType::NotFound, "KID not found"))?;
            let domain = config.domain.as_str();
            let jwks: JwkSet = Client::new()
                .get(
                    Uri::builder()
                        .scheme("https")
                        .authority(domain)
                        .path_and_query("/.well-known/jwks.json")
                        .build()
                        .map_err(|_|ApiError::new(ErrorType::InternalServerError, "Unable to find well known"))?
                )
                .send()
                .await.map_err(|e| ApiError::new(ErrorType::NotFound, "JWKS Not Found")
            )?
                .json()
                .await.map_err(|_| ApiError::new(ErrorType::NotFound, "Can't deserialize"))?;

            let jwk = jwks
                .find(&kid)
                .ok_or_else(|| ApiError::new(ErrorType::NotFound, "No JWK"))?;

            match jwk.clone().algorithm {
                AlgorithmParameters::RSA(ref rsa) => {
                    let mut validation = Validation::new(Algorithm::RS256);
                    validation.set_audience(&[config.audience]);
                    validation.set_issuer(&[Uri::builder()
                        .scheme("https")
                        .authority(domain)
                        .path_and_query("/")
                        .build()
                        .unwrap()]);
                    let key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e)
                        .map_err(|_| ApiError::new(ErrorType::NotFound, "No Key"))?;
                    let token =
                        decode::<Claims>(token, &key, &validation).map_err(|_| ApiError::new(ErrorType::NotFound, "Can't deserialize"))?;
                    Ok(token.claims)
                }
                algorithm => Err(ApiError::new(ErrorType::NotFound, "Can't deserialize").into()),
            }
        })
    }
}