use actix_web::web;

use super::controller;
use crate::middleware::auth;

pub fn config(cfg: &mut web::ServiceConfig) -> () {
    cfg
        .service(
            web::scope("")
                .wrap(auth::Auth)
                .service(controller::get_transactions_for_card)
                .service(controller::get_all_transactions)
        );
}