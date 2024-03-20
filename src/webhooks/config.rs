use actix_web::web;
use super::controller;

pub fn config(cfg: &mut web::ServiceConfig) -> () {
    cfg
        .service(controller::lithic_asa_webhook);
}
