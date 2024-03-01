// TODO: Remove
#![allow(warnings)]
extern crate diesel;
extern crate diesel_migrations;
extern crate diesel_async;
#[macro_use]
extern crate log;
extern crate num;
extern crate num_derive;
extern crate env_logger;
extern crate console_subscriber;
extern crate uuidv7;

use std::io::ErrorKind;
use std::str::FromStr;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use dotenv::dotenv;
use uuid::Uuid;
use crate::lithic_service::service::{LithicService, LithicServiceTrait};
use crate::passthrough_card::crypto::encrypt_pin;


mod adyen_service;
mod asa;
mod util;
mod api_error;
mod constant;
mod lithic_service;
mod credit_card_type;
mod rule_engine;
mod category;
mod membership;
mod transaction;
mod passthrough_card;
mod schema;
mod user;
mod wallet;
mod middleware;
mod webhooks;
mod charge_engine;
mod auth;

#[cfg(test)]
mod test;
#[cfg(test)]
mod test_helper;
mod data_error;
mod service_error;
mod environment;


async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

//#[actix_web::main]
// TODO: why does tokio vs actix cause inner requests not to hang
#[tokio::main(flavor = "multi_thread", worker_threads = 32)]
async fn main() -> std::io::Result<()> {
    console_subscriber::init();
    dotenv().ok();
    let orig_id = uuidv7::create();
    println!("{:?}", orig_id.to_string());
    let id = Uuid::from_str(orig_id.as_str()).expect("should serialize");
    println!("{:?}", id.to_string());
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    util::db::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(web::scope("/user").configure(user::config::config))
            .service(web::scope("/wallet").configure(wallet::config::config))
            .service(web::scope("/webhook").configure(webhooks::config::config))
            .service(web::scope("/auth").configure(auth::config::config))
            .service(web::scope("/passthrough").configure(passthrough_card::config::config))
            .service(web::scope("/credit-card-type").configure(credit_card_type::config::config))
            .route("/hey/", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    //lithic.deregister_webhook(resp.token).await.map_err(|e| std::io::Error::new(ErrorKind::Other, e.message))?;

    Ok(())
}