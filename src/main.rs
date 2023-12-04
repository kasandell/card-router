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
mod asa_request;
mod util;
mod api_error;
mod constant;
mod credit_card_type;
mod rule_engine;
mod category;
mod membership;
mod transaction;
mod schema;
mod user;
mod wallet;
mod middleware;
mod webhooks;

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
    //let pool = util::db::establish_connection();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    util::db::init();

    /*
    let state = {
        let pool = util::db::establish_connection();
        use middleware::state::AppState;
        AppState::new(pool)
    };
    */

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            //.wrap(api_key_validation::ApiKeyValidation)
            //.app_data(web::Data::new(state.clone()))
            .service(web::scope("/user").configure(user::config::config))
            .service(web::scope("/wallet").configure(wallet::config::config))
            .route("/hey/", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}