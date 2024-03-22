use actix_web::{
    web,
    get,
    post,
    HttpResponse,
};
use uuid::Uuid;

use super::error::WalletError;
use crate::middleware::services::Services;
use crate::user::entity::User;
use crate::wallet::service::WalletService;
use crate::wallet::entity::DisplayableCardInfo;
use crate::wallet::response::{WalletAddCardSuccessResponse, WalletCardAttemptResponse};
use super::{
    request, 
    entity::{
        NewCard,
        Wallet
    }
};

#[post("/register-card-attempt/")]
async fn register_new_card_attempt(
    user: web::ReqData<User>,
    info: web::Json<request::RegisterAttemptRequest>,
    services: web::Data<Services>
) -> Result<HttpResponse, WalletError> {
    let user = user.into_inner();
    let info = info.into_inner();
    let wca = services.wallet_service.clone().register_new_attempt(
        &user,
        &info
    ).await?;
    Ok(HttpResponse::Ok().json(
        wca
    ))
}

#[post("/match-card/")]
async fn match_card(
    user: web::ReqData<User>,
    info: web::Json<request::MatchRequest>,
    services: web::Data<Services>
) -> Result<HttpResponse, WalletError> {
    let user = user.into_inner();
    let info = info.into_inner();
    tracing::info!("{:?}", &info);
    let wca = services.wallet_service.clone().match_card(
        &user,
        &info
    ).await?;
    Ok(HttpResponse::Ok().json(
        WalletAddCardSuccessResponse {
            public_id: wca.public_id.to_string()
        }
    ))
}

#[get("/list-cards/")]
async fn list_cards(
    user: web::ReqData<User>, // should extract from extensions
    services: web::Data<Services>
) -> Result<HttpResponse, WalletError> {
    let user = user.into_inner();
    let cards: Vec<DisplayableCardInfo> = services.wallet_service.wallet_dao.clone().find_all_for_user_with_card_info(
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