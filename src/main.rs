// TODO: Remove
#![allow(warnings)]
extern crate diesel;
extern crate diesel_migrations;
#[macro_use]
extern crate log;
extern crate num;
extern crate num_derive;
extern crate env_logger;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use dotenv::dotenv;


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


async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
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
    .await
}