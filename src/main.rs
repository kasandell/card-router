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
use uuid::Uuid;
use crate::auth::entity::{Claims, Auth0Config};

use tracing::subscriber::set_global_default;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_actix_web::TracingLogger;


mod adyen;
mod asa;
mod util;
mod constant;
mod lithic;
mod credit_card_type;
mod rule;
mod category;
mod membership;
mod ledger;
mod passthrough_card;
mod schema;
mod user;
mod wallet;
mod middleware;
mod webhooks;
mod charge;
mod auth;

#[cfg(test)]
mod test_helper;
mod environment;
mod footprint;
mod error;


async fn manual_hello(claims: Claims) -> impl Responder {
    tracing::info!("{:?}", &claims);
    HttpResponse::Ok().body("Hey there!")
}

//#[actix_web::main]
// TODO: why does tokio vs actix cause inner requests not to hang
#[tokio::main(flavor = "multi_thread", worker_threads = 32)]
async fn main() -> std::io::Result<()> {
    //std::env::set_var("RUST_LOG", "info");
    //console_subscriber::init();
    LogTracer::init().expect("Failed to set logger");
    /*
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    util::db::init().await;

     */

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    // The `with` method is provided by `SubscriberExt`, an extension
    // trait for `Subscriber` exposed by `tracing_subscriber`
    let stdout_log = tracing_subscriber::fmt::layer()
        .with_span_events(FmtSpan::CLOSE)
        .compact();
        //.pretty();
    let subscriber = Registry::default()
        .with(env_filter)
        .with(stdout_log);
    // `set_global_default` can be used by applications to specify
    // what subscriber should be used to process spans.
    set_global_default(subscriber).expect("Failed to set subscriber");
    tracing::warn!("TEST");

    dotenv().ok();

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
    Ok(())
}