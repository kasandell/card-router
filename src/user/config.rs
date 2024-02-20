use super::controller;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) -> () {
    if cfg!(test) {
        cfg
            .service(
                web::scope("")
                    .service(controller::create)
                    .service(controller::list)
                    .service(controller::find)
            );

    } else {
        cfg
            .service(controller::create)
            .service(
                web::scope("")
                    .wrap(crate::middleware::auth::Auth)
                    .service(controller::list)
                    .service(controller::find)
            );
    }

}
