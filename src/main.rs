#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
extern crate num;
#[macro_use]
extern crate num_derive;
extern crate env_logger;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use dotenv::dotenv;

mod util;
mod api_error;
mod constant;
mod card;
mod category;
mod membership;
mod transaction;
mod schema;
mod user;
mod wallet;
mod middleware;

#[cfg(test)]
mod test;


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
            .route("/hey/", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}