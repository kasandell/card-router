use actix_web::{
    web,
    get,
    post,
    HttpResponse,
};

use crate::api_error::ApiError;
use crate::middleware::services::Services;
use crate::user::entity::User;
use crate::wallet::engine::Engine;
use crate::wallet::entity::DisplayableCardInfo;
use crate::wallet::response::WalletCardAttemptResponse;
use super::{
    request, 
    entity::{
        NewCard,
        Wallet
    }
};


#[post("/add-card/")]
async fn add_card(
    user: web::ReqData<User>,
    info: web::Json<request::AddCardRequest>,
    services: web::Data<Services>
) -> Result<HttpResponse, ApiError> {
    println!("IN REQUEST");
    let user = user.into_inner();
    println!("GOT USER");
    let info = info.into_inner();
    println!("GOT ENGINE");
    let (wca, payment_response) = services.wallet_engine.clone().register_attempt_and_send_card_to_adyen(
        &user,
        &info
    ).await?;
    println!("registered attempt");
    let match_from_response = services.wallet_engine.clone().attempt_match_from_response(&payment_response).await;
    println!("done registering");
    Ok(HttpResponse::Ok().json(
        WalletCardAttemptResponse {
            public_id: wca.public_id
        }
    ))
}

#[post("/register-card-attempt/")]
async fn register_new_card_attempt(
    user: web::ReqData<User>,
    info: web::Json<request::RegisterAttemptRequest>,
    services: web::Data<Services>
) -> Result<HttpResponse, ApiError> {
    let user = user.into_inner();
    let info = info.into_inner();
    let wca = services.wallet_engine.clone().attempt_register_new_attempt(
        &user,
        &info
    ).await?;
    Ok(HttpResponse::Ok().json(
        WalletCardAttemptResponse {
            public_id: wca.public_id
        }
    ))
}

#[get("/list-cards/")]
async fn list_cards(
    user: web::ReqData<User>, // should extract from extensions
    services: web::Data<Services>
) -> Result<HttpResponse, ApiError> {
    let user = user.into_inner();
    let cards: Vec<DisplayableCardInfo> = services.wallet_engine.wallet_dao.clone().find_all_for_user_with_card_info(
        &user
    ).await?
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