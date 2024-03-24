use super::controller;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) -> () {
    cfg
        .service(controller::create)
        .service(
            web::scope("")
                .wrap(crate::middleware::auth::Auth)
        );

}
