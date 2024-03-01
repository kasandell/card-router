use actix_web::{put, post, get, HttpResponse, web};
use crate::api_error::ApiError;
use crate::credit_card_type::response::CardTypeResponse;
use crate::passthrough_card::response::HasActiveResponse;
use crate::user::entity::User;
use super::entity::CreditCard;


#[get("/list/")]
async fn list_cards() -> Result<HttpResponse, ApiError>{
    warn!("Hi Im here");
    let cards: Vec<CardTypeResponse> = CreditCard::list_all_card_types().await?
        .iter()
        .map(|card| CardTypeResponse {
            public_id: card.0.public_id,
            card_name: card.0.name.clone(),
            issuer_name: card.2.name.clone(),
            card_type: card.1.name.clone(),
            card_image_url: card.0.card_image_url.clone(),
        }).collect();
    Ok(
        HttpResponse::Ok().json(
            &cards
        )
    )

}