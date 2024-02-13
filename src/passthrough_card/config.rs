use actix_web::web;
use super::controller;

pub fn config(cfg: &mut web::ServiceConfig) -> () {
    if cfg!(test) {
    } else {
        cfg
            .service(
                web::scope("")
                    .wrap(crate::middleware::auth::Auth)
                    .service(controller::create_card)
                    .service(controller::get_card)
                    .service(controller::active_card)
                    .service(controller::pause_card)
                    .service(controller::unpause_card)
                    .service(controller::cancel_card)
            );
    }
}
