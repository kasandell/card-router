use actix_web::{get, HttpResponse, web};
use crate::credit_card_type::error::CreditCardTypeError;
use crate::credit_card_type::response::CardTypeResponse;
use crate::credit_card_type::service::CreditCardServiceTrait;
use crate::middleware::services::Services;


#[get("/list/")]
async fn list_cards(
    services: web::Data<Services>
) -> Result<HttpResponse, CreditCardTypeError>{
    let cards: Vec<CardTypeResponse> = services.credit_card_service.clone().list_all_card_types().await?
        .into_iter()
        .map(|card| CardTypeResponse {
            public_id: card.public_id,
            card_name: card.name,
            issuer_name: card.credit_card_issuer_name,
            card_type: card.credit_card_type_name,
            card_image_url: card.card_image_url
        }).collect();
    Ok(
        HttpResponse::Ok().json(
            &cards
        )
    )

}