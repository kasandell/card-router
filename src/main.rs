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

use std::str::FromStr;
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
mod environment;
mod footprint;
mod error;
#[cfg(test)]
mod test_helper;
mod common;


async fn manual_hello(claims: Claims) -> impl Responder {
    tracing::info!("{:?}", &claims);
    HttpResponse::Ok().body("Hey there!")
}


fn create_otlp_tracer() -> Option<opentelemetry_sdk::trace::Tracer> {
    if !std::env::vars().any(|(name, _)| name.starts_with("OTEL_")) {
        return None;
    }
    let protocol = std::env::var("OTEL_EXPORTER_OTLP_PROTOCOL").unwrap_or("grpc".to_string());

    let tracer = opentelemetry_otlp::new_pipeline().tracing();
    let headers = parse_otlp_headers_from_env();

    let tracer = match protocol.as_str() {
        "grpc" => {
            let mut exporter = opentelemetry_otlp::new_exporter()
                .tonic()
                .with_metadata(metadata_from_headers(headers));

            // Check if we need TLS
            if let Ok(endpoint) = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT") {
                if endpoint.starts_with("https") {
                    exporter = exporter.with_tls_config(Default::default());
                }
            }
            tracer.with_exporter(exporter)
        }
        "http/protobuf" => {
            let exporter = opentelemetry_otlp::new_exporter()
                .http()
                .with_headers(headers.into_iter().collect());
            tracer.with_exporter(exporter)
        }
        p => panic!("Unsupported protocol {}", p),
    };

    Some(
        tracer
            .install_batch(opentelemetry_sdk::runtime::Tokio)
            .unwrap(),
    )
}

fn metadata_from_headers(headers: Vec<(String, String)>) -> tonic::metadata::MetadataMap {
    use tonic::metadata;

    let mut metadata = metadata::MetadataMap::new();
    headers.into_iter().for_each(|(name, value)| {
        let value = value
            .parse::<metadata::MetadataValue<metadata::Ascii>>()
            .expect("Header value invalid");
        metadata.insert(metadata::MetadataKey::from_str(&name).unwrap(), value);
    });
    metadata
}

// Support for this has now been merged into opentelemetry-otlp so check next release after 0.14
fn parse_otlp_headers_from_env() -> Vec<(String, String)> {
    let mut headers = Vec::new();

    if let Ok(hdrs) = std::env::var("OTEL_EXPORTER_OTLP_HEADERS") {
        hdrs.split(',')
            .map(|header| {
                header
                    .split_once('=')
                    .expect("Header should contain '=' character")
            })
            .for_each(|(name, value)| headers.push((name.to_owned(), value.to_owned())));
    }
    headers
}

//#[actix_web::main]
// TODO: why does tokio vs actix cause inner requests not to hang
#[tokio::main(flavor = "multi_thread", worker_threads = 64)]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    LogTracer::init().expect("Failed to set logger");

    let telemetry_layer =
        create_otlp_tracer().map(|t| tracing_opentelemetry::layer().with_tracer(t));

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    let stdout_log = tracing_subscriber::fmt::layer()
        .with_span_events(FmtSpan::CLOSE)
        .compact();
    let subscriber = Registry::default()
        .with(env_filter)
        .with(stdout_log)
        .with(telemetry_layer);
    set_global_default(subscriber).expect("Failed to set subscriber");
    tracing::warn!("TEST");


    HttpServer::new(move || {
        let services = middleware::services::Services::new();
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
            .service(
                web::scope("/")
            )
            .route("/hey/", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    //shutdown_tracer_provider();
    global::shutdown_tracer_provider();
    Ok(())
}