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
use crate::constant;
use super::error::AuthError;

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

    #[tracing::instrument(skip_all)]
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let Some(config) = req.app_data::<Auth0Config>().cloned() else {
            return Box::pin(async move {
                return Err(AuthError::Unauthorized("Config not found".into()).into());
            })
        };
        let extractor = BearerAuth::extract(req);
        Box::pin(async move {
            let credentials = extractor.await.map_err(
                |_| AuthError::Unauthorized("Can't get credentials".into())
            )?;
            let token = credentials.token();
            let header = decode_header(token).map_err(|_| AuthError::Unauthorized("Can't decode".into()))?;
            let kid = header.kid.ok_or_else(||  AuthError::Unauthorized("Can't get KID".into()))?;
            let domain = config.domain.as_str();
            let jwks: JwkSet = Client::new()
                .get(
                    Uri::builder()
                        .scheme("https")
                        .authority(domain)
                        .path_and_query("/.well-known/jwks.json")
                        .build()
                        .map_err(|_| AuthError::Unauthorized("Can't get well known".into()))?
                )
                .send()
                .await.map_err(|e|  AuthError::Unauthorized("JWKS Not found".into())
            )?
                .json()
                .await.map_err(|_|  AuthError::Unauthorized("Can't deserialize JWKS".into()))?;

            let jwk = jwks
                .find(&kid)
                .ok_or_else(||  AuthError::Unauthorized("Can't find JWK".into()))?;

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
                        .map_err(|_|  AuthError::Unauthorized("Can't get decoding key".into()))?;
                    let token =
                        decode::<Claims>(token, &key, &validation).map_err(|_|  AuthError::Unauthorized("Can't get claims".into()))?;
                    Ok(token.claims)
                }
                algorithm => Err( AuthError::Unauthorized("Can't decode".into()).into()),
            }
        })
    }
}