use std::any::Any;
use actix_web::{put, post, get, HttpResponse, web};
use uuid::Uuid;
use crate::api_error::ApiError;
use crate::passthrough_card::constant::PassthroughCardStatus;
use crate::passthrough_card::response::{HasActiveResponse, PassthroughCardResposnse};
use crate::user::entity::User;
use super::engine::Engine;

#[post("/create-card/")]
async fn create_card(
    user: web::ReqData<User>
) -> Result<HttpResponse, ApiError> {
    println!("HEI");
    let user = user.into_inner();
    let engine = Engine::new();
    println!("HI");
    let card = engine.issue_card_to_user(&user, "1234".to_string()).await?;
    Ok(
        HttpResponse::Ok().json(
            PassthroughCardResposnse {
                public_id: card.public_id.clone(),
                card_status: card.passthrough_card_status.clone(),
                card_type: card.passthrough_card_type.clone(),
                last_four: card.last_four.clone(),
            }
        )
    )
}

#[get("/get-card/")]
async fn get_card(
    user: web::ReqData<User>
) -> Result<HttpResponse, ApiError> {
    let user = user.into_inner();
    let engine = Engine::new();
    return if let Some(card) = engine.get_active_card_for_user(&user).await? {
        println!("FOUND A CARD");
        Ok(
            HttpResponse::Ok().json(
                PassthroughCardResposnse {
                    public_id: card.public_id.clone(),
                    card_status: card.passthrough_card_status.clone(),
                    card_type: card.passthrough_card_type.clone(),
                    last_four: card.last_four.clone(),
                }
            )
        )
    } else {
        println!("DID NOT FIND A CARD");
        Ok(
            HttpResponse::NotFound().finish()
        )
    }
}

#[get("/active-card/")]
async fn active_card(
    user: web::ReqData<User>
) -> Result<HttpResponse, ApiError> {
    let user = user.into_inner();
    let engine = Engine::new();
    let has_active = engine.user_has_active_card(&user).await?;
    Ok(
        HttpResponse::Ok().json(
            HasActiveResponse {
                has_active
            }
        )
    )
}

#[put("/pause-card/")]
async fn pause_card(
    user: web::ReqData<User>
) -> Result<HttpResponse, ApiError> {
    let user = user.into_inner();
    let engine = Engine::new();
    engine.update_card_status(&user, PassthroughCardStatus::PAUSED).await?;
    Ok(
        HttpResponse::Ok().finish()
    )
}

#[put("/unpause-card/")]
async fn unpause_card(
    user: web::ReqData<User>
) -> Result<HttpResponse, ApiError> {
    let user = user.into_inner();
    let engine = Engine::new();
    engine.update_card_status(&user, PassthroughCardStatus::OPEN).await?;
    Ok(
        HttpResponse::Ok().finish()
    )
}

#[put("/cancel-card/")]
async fn cancel_card(
    user: web::ReqData<User>
) -> Result<HttpResponse, ApiError> {
    let user = user.into_inner();
    let engine = Engine::new();
    engine.update_card_status(&user, PassthroughCardStatus::CLOSED).await?;
    Ok(
        HttpResponse::Ok().finish()
    )
}

