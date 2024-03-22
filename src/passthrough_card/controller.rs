use actix_web::{put, post, get, HttpResponse, web};
use crate::passthrough_card::constant::PassthroughCardStatus;
use crate::passthrough_card::response::{HasActiveResponse, PassthroughCardResposnse};
use crate::user::entity::User;
use crate::middleware::services::Services;
use super::error::PassthroughCardError;

#[post("/create-card/")]
async fn create_card(
    user: web::ReqData<User>,
    services: web::Data<Services>
) -> Result<HttpResponse, PassthroughCardError> {
    // TODO: need to pass in pin
    let user = user.into_inner();
    // TODO: pin needs to be from frontend
    let card = services.passthrough_card_service.clone().issue_card_to_user(&user, "1234").await?;
    Ok(
        HttpResponse::Ok().json(
            PassthroughCardResposnse {
                public_id: card.public_id.clone(),
                card_status: card.passthrough_card_status.to_string(),
                card_type: card.passthrough_card_type.to_string(),
                last_four: card.last_four.clone(),
            }
        )
    )
}

#[get("/get-card/")]
async fn get_card(
    user: web::ReqData<User>,
    services: web::Data<Services>
) -> Result<HttpResponse, PassthroughCardError> {
    let user = user.into_inner();
    return if let Some(card) = services.passthrough_card_service.clone().get_active_card_for_user(&user).await? {
        Ok(
            HttpResponse::Ok().json(
                PassthroughCardResposnse {
                    public_id: card.public_id.clone(),
                    card_status: card.passthrough_card_status.to_string(),
                    card_type: card.passthrough_card_type.to_string(),
                    last_four: card.last_four.clone(),
                }
            )
        )
    } else {
        Ok(
            HttpResponse::NotFound().finish()
        )
    }
}

#[get("/active-card/")]
async fn active_card(
    user: web::ReqData<User>,
    services: web::Data<Services>,
) -> Result<HttpResponse, PassthroughCardError> {
    let user = user.into_inner();
    let has_active = services.passthrough_card_service.clone().user_has_active_card(&user).await?;
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
    user: web::ReqData<User>,
    services: web::Data<Services>
) -> Result<HttpResponse, PassthroughCardError> {
    let user = user.into_inner();
    services.passthrough_card_service.clone().update_card_status(&user, PassthroughCardStatus::Paused).await?;
    Ok(
        HttpResponse::Ok().finish()
    )
}

#[put("/unpause-card/")]
async fn unpause_card(
    user: web::ReqData<User>,
    services: web::Data<Services>,
) -> Result<HttpResponse, PassthroughCardError> {
    let user = user.into_inner();
    services.passthrough_card_service.clone().update_card_status(&user, PassthroughCardStatus::Open).await?;
    Ok(
        HttpResponse::Ok().finish()
    )
}

#[put("/cancel-card/")]
async fn cancel_card(
    user: web::ReqData<User>,
    services: web::Data<Services>,
) -> Result<HttpResponse, PassthroughCardError> {
    let user = user.into_inner();
    services.passthrough_card_service.clone().update_card_status(&user, PassthroughCardStatus::Closed).await?;
    Ok(
        HttpResponse::Ok().finish()
    )
}

