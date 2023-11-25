use actix_web::{
    web,
    get,
    post,
    HttpResponse,
};

use crate::api_error::ApiError;
use crate::user::entity::User;
use super::{
    request, 
    entity::{
        NewCard,
        Wallet
    }
};


#[post("/add-card/")]
async fn add_card(info: web::Json<request::AddCardRequest>) -> Result<HttpResponse, ApiError> {
    let new_card = NewCard {
        user_id: 1, // TODO: populate from request
        stripe_payment_method_id: info.into_inner().stripe_payment_method_id,
        credit_card_id: 1
    };
    let inserted_card = Wallet::insert_card(new_card)?;
    Ok(HttpResponse::Ok().json(inserted_card))
}

#[get("/list-cards/")]
async fn list_cards() -> Result<HttpResponse, ApiError> {
    let cards = Wallet::find_all_for_user(
        User::find_by_internal_id(1)?
    )?;
    Ok(HttpResponse::Ok().json(cards))
}