use actix_web::web;
use super::controller;

pub fn config(cfg: &mut web::ServiceConfig) -> () {
    if cfg!(test) {
    } else {
        cfg
            .service(
                web::scope("")
                    .wrap(crate::middleware::auth::Auth)
                    .service(controller::list_cards)
            );
    }
}
