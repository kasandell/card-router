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
use secrecy::ExposeSecret;
use crate::configuration::configuration::get_configuration_sync;
use crate::constant;
use super::error::AuthError;

#[derive(Clone, Deserialize)]
pub struct Auth0Config {
    audience: String,
    domain: String,
}

impl Default for Auth0Config {
    fn default() -> Self {
        let config = get_configuration_sync().expect("config should exist");
        Self {
            audience: config.auth0.audience.expose_secret().clone(),
            domain: config.auth0.domain.expose_secret().clone(),
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

    #[cfg_attr(feature="trace-detail", tracing::instrument(skip_all))]
    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        tracing::info!("Converting claims from request");
        let Some(config) = req.app_data::<Auth0Config>().cloned() else {
            tracing::error!("No auth0 config found");
            return Box::pin(async move {
                return Err(AuthError::Unauthorized("Config not found".into()).into());
            })
        };
        tracing::info!("Extracting token");
        let extractor = BearerAuth::extract(req);
        Box::pin(async move {
            tracing::info!("Extracting bearer auth");
            let credentials = extractor.await.map_err(
                |_| {
                    tracing::error!("Credentials not found in request");
                    AuthError::Unauthorized("Can't get credentials".into())
                }
            )?;
            let token = credentials.token();
            tracing::info!("Extracting header");
            let header = decode_header(token).map_err(|_| {
                tracing::error!("Failed to decode header");
                AuthError::Unauthorized("Can't decode".into())
            })?;
            tracing::info!("Getting KID");
            let kid = header.kid.ok_or_else(|| {
                tracing::error!("Unable to get KID from header");
                AuthError::Unauthorized("Can't get KID".into())
            })?;
            let domain = config.domain.as_str();
            tracing::info!("Getting JWK's with authority={}", domain);
            let jwks: JwkSet = Client::new()
                .get(
                    Uri::builder()
                        .scheme("https")
                        .authority(domain)
                        .path_and_query("/.well-known/jwks.json")
                        .build()
                        .map_err(|_| {
                            tracing::error!("Unable to get well-known file");
                            AuthError::Unauthorized("Can't get well known".into())
                        })?
                )
                .send()
                .await.map_err(|e| {
                    tracing::error!("Could not get jwks");
                    AuthError::Unauthorized("JWKS Not found".into())
                })?
                .json()
                .await.map_err(|_| {
                    tracing::error!("Unable to deserialize JWKS");
                    AuthError::Unauthorized("Can't deserialize JWKS".into())
                })?;

            tracing::info!("Finding jwk for kid");
            let jwk = jwks
                .find(&kid)
                .ok_or_else(|| {
                    tracing::error!("Unable to find JWK");
                    AuthError::Unauthorized("Can't find JWK".into())
                })?;

            tracing::info!("Matching jwk from algorithm");
            match jwk.clone().algorithm {
                AlgorithmParameters::RSA(ref rsa) => {
                    tracing::info!("Matched RSA");
                    tracing::info!("Creating validation");
                    let mut validation = Validation::new(Algorithm::RS256);
                    validation.set_audience(&[config.audience]);
                    validation.set_issuer(&[Uri::builder()
                        .scheme("https")
                        .authority(domain)
                        .path_and_query("/")
                        .build()
                        .unwrap()]);
                    tracing::info!("Creating key");
                    let key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e)
                        .map_err(|_|  AuthError::Unauthorized("Can't get decoding key".into()))?;
                    tracing::info!("Creating token");
                    let token =
                        decode::<Claims>(token, &key, &validation).map_err(|_|  AuthError::Unauthorized("Can't get claims".into()))?;
                    Ok(token.claims)
                }
                algorithm => {
                    tracing::error!("Unrecognized algorithm");
                    Err( AuthError::Unauthorized("Can't decode".into()).into())
                }
            }
        })
    }
}