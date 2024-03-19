use actix_web::web;

use super::controller;
use crate::middleware::auth;

pub fn config(cfg: &mut web::ServiceConfig) -> () {
    cfg
        .service(
            web::scope("")
                .wrap(auth::Auth)
                .service(controller::list_cards)
                .service(controller::register_new_card_attempt)
                .service(controller::add_card)
                .service(controller::match_card)
        );
        //.service(controller::add_card)
        //.service(controller::list_cards);
}