use actix_web::web;

use super::controller;
use crate::middleware::auth;

pub fn config(cfg: &mut web::ServiceConfig) -> () {
    cfg
        .service(
            web::scope("")
                .wrap(auth::Auth)
                .service(controller::add_card)
                .service(controller::list_cards)

        );
        //.service(controller::add_card)
        //.service(controller::list_cards);
}