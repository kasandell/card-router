use super::controller;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) -> () {
    cfg
        .service(
            web::scope("")
                .wrap(crate::middleware::auth::Auth)
                .service(controller::list)
                .service(controller::find)
                .service(controller::create)
        );
}
