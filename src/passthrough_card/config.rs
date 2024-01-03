use actix_web::web;
use super::controller;

pub fn config(cfg: &mut web::ServiceConfig) -> () {
    if cfg!(test) {
    } else {
        cfg
            .wrap(crate::middleware::auth::Auth)
            .service(controller::create_card)
            .service(controller::pause_card)
            .service(controller::unpause_card)
            .service(controller::cancel_card);
    }
}
