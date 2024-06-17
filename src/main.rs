// TODO: Remove
#![allow(warnings)]
extern crate diesel;
extern crate diesel_migrations;
extern crate diesel_async;
#[macro_use]
extern crate log;
extern crate num;
extern crate num_derive;
extern crate console_subscriber;
extern crate uuidv7;

use std::fmt;
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::Arc;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use crate::auth::entity::{Claims, Auth0Config};
use tracing::subscriber::set_global_default;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_actix_web::TracingLogger;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry::global;
use crate::configuration::configuration::get_configuration_sync;
use crate::error::data_error::DataError;
use crate::otel::otel::create_otlp_tracer;
#[cfg(not(feature = "no-redis"))]
use crate::redis::key::Key;
#[cfg(not(feature = "no-redis"))]
use crate::redis::services::{
    RedisService,
    RedisServiceTrait
};


mod adyen;
mod asa;
mod util;
mod constant;
mod lithic;
mod credit_card_type;
mod rule;
mod category;
mod ledger;
mod passthrough_card;
mod schema;
mod user;
mod wallet;
mod middleware;
mod webhooks;
mod charge;
mod auth;
mod footprint;
mod error;
#[cfg(test)]
mod test_helper;
mod common;
#[cfg(not(feature = "no-redis"))]
mod redis;
mod configuration;
mod otel;
mod user_transaction;
mod pagination;


async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("live")
}




#[tracing::instrument]
async fn ping_db() -> Result<(), DataError>{
    let conn = util::db::connection().await?;
    Ok(())
}

// TODO: why does tokio vs actix cause inner requests not to hang
#[tokio::main(flavor = "multi_thread", worker_threads = 64)]
//#[tokio::main(flavor = "current_thread")]
//#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //dotenv().ok();
    let configuration = get_configuration_sync().expect("should load config");

    LogTracer::init().expect("Failed to set logger");

    let telemetry_layer =
        create_otlp_tracer(&configuration.otel).map(|t| tracing_opentelemetry::layer().with_tracer(t));

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    // TODO: this absolutely shits on our runtime

    #[cfg(feature="logs-to-stdout")]
    let stdout_log = tracing_subscriber::fmt::layer()
        .with_span_events(FmtSpan::CLOSE)
        .with_target(false) // Do not print the target (module path)
        .with_level(false)  // Do not print the log level
        .with_thread_ids(false) // Do not print thread IDs
        .with_thread_names(false) // Do not print thread names
        .event_format(
            tracing_subscriber::fmt::format().without_time().pretty()
        ); // Do not print timestamp

    #[cfg(feature="logs-to-stdout")]
    let subscriber = Registry::default()
        .with(env_filter)
        .with(telemetry_layer)
        .with(stdout_log);

    #[cfg(not(feature="logs-to-stdout"))]
    let subscriber = Registry::default()
        .with(env_filter)
        .with(telemetry_layer);

    set_global_default(subscriber).expect("Failed to set subscriber");
    tracing::warn!("TEST");
    tracing::warn!("{:?}", num_cpus::get_physical());


    let res = ping_db().await.expect("No issue");



    HttpServer::new(move || {
        let configuration = get_configuration_sync().expect("gets configuration");
        let services = middleware::services::Services::new(&configuration);
        let auth0_config = Auth0Config::default();
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(services.clone()))
            .app_data(auth0_config.clone())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(web::scope("/user").configure(user::config::config))
            .service(web::scope("/wallet").configure(wallet::config::config))
            .service(web::scope("/webhook").configure(webhooks::config::config))
            .service(web::scope("/passthrough").configure(passthrough_card::config::config))
            .service(web::scope("/credit-card-type").configure(credit_card_type::config::config))
            .service(web::scope("/transactions").configure(user_transaction::config::config))
            .service(
                web::scope("/")
            )
            .route("/health-check/", web::get().to(health_check))
    })
    .bind(("0.0.0.0", configuration.application.port))?
    .run()
    .await?;

    global::shutdown_tracer_provider();
    Ok(())
}