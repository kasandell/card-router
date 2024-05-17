use actix_web::{
    web,
    get,
    post,
    HttpResponse,
};

use super::error::WalletError;
use crate::middleware::services::Services;
use crate::user::model::UserModel as User;
use crate::wallet::service::{WalletService, WalletServiceTrait};
use crate::wallet::response::{DisplayableCardInfo, UpdateStatusResponse};
use crate::wallet::response::WalletAddCardSuccessResponse;
use super::{
    request, 
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
    let cards: Vec<DisplayableCardInfo> = services.wallet_service.clone().find_all_for_user_with_card_info(
        &user
    ).await?
        .into_iter()
        .map(|card| card.into())
        .collect();
    Ok(HttpResponse::Ok().json(cards))
}


#[post("/update_status/")]
async fn update_status(
    user: web::ReqData<User>,
    info: web::Json<request::UpdateStatusRequest>,
    services: web::Data<Services>
) -> Result<HttpResponse, WalletError> {
    let user = user.into_inner();
    let info = info.into_inner();
    tracing::info!("{:?}", &info);
    let card = services.wallet_service.clone().update_card_status(
        &user,
        &info.wallet_card_public_id,
        info.status
    ).await?;

    Ok(HttpResponse::Ok().json(
        UpdateStatusResponse{
            public_id: card.public_id.clone(),
            status: card.status.clone()
        }
    ))
}
