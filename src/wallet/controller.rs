use actix_web::{
    web,
    get,
    post,
    HttpResponse,
};

use crate::api_error::ApiError;
use crate::user::entity::User;
use crate::wallet::entity::DisplayableCardInfo;
use super::{
    request, 
    entity::{
        NewCard,
        Wallet
    }
};


// TODO: this needs to be adjusted to create a card attempt
#[post("/add-card/")]
async fn add_card(info: web::Json<request::AddCardRequest>) -> Result<HttpResponse, ApiError> {
    let new_card = NewCard {
        user_id: 1, // TODO: populate from request
        payment_method_id: info.into_inner().stripe_payment_method_id,
        credit_card_id: 1,
        wallet_card_attempt_id: 0,
    };
    let inserted_card = Wallet::insert_card(new_card)?;
    Ok(HttpResponse::Ok().json(inserted_card))
}

#[get("/list-cards/")]
async fn list_cards(
    user: web::ReqData<User> // should extract from extensions
) -> Result<HttpResponse, ApiError> {
    let user = user.into_inner();
    /*
    let cards = Wallet::find_all_for_user(
        &(User::find_by_internal_id(1)?)
    )?;
     */
    let cards: Vec<DisplayableCardInfo> = Wallet::find_all_for_user_with_card_info(
        &user //(User::find_by_internal_id(1)?)
    )?
        .iter()
        .map(|card| DisplayableCardInfo {
            public_id: card.0.public_id,
            created_at: card.0.created_at,
            card_name: card.1.name.clone(),
            issuer_name: card.3.name.clone(),
            card_type: card.2.name.clone(),
            card_image_url: card.1.card_image_url.clone(),
        })
        .collect();
    Ok(HttpResponse::Ok().json(cards))
}

// TODO: need a remove card endpoint (mark as deleted)