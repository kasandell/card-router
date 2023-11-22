use super::controller;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) -> () {
    cfg
    .service(controller::list)
    .service(controller::find)
    .service(controller::create);
    /*
    cfg.service(web::resource("{action_id}/approve/").route(web::post().to(controller::approve)))
        .service(web::resource("{action_id}/deny/").route(web::post().to(controller::deny)))
        .service(
            web::resource("create/")
                .route(web::post().to(controller::create_action))
                .wrap(IdempotencyKeyValidation),
        )
        .service(web::resource("list/").route(web::get().to(controller::list)))
        .service(web::resource("{action_id}/").route(web::get().to(controller::get)));
    */
}
