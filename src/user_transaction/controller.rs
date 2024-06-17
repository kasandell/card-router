use actix_web::{get, HttpResponse, web};
use uuid::Uuid;
use crate::middleware::services::Services;
use crate::pagination::request::PaginationRequest;
use crate::user::model::UserModel;
use crate::user_transaction::error::UserTransactionError;
use crate::user_transaction::request::CursorQueryParams;
use crate::user_transaction::response::{TransactionsForUserResponse, TransactionsForWalletCardResponse};
use crate::user_transaction::service::UserTransactionServiceTrait;

#[get("/")]
async fn get_all_transactions(
    user: web::ReqData<UserModel>,
    query: web::Query<CursorQueryParams>,
    services: web::Data<Services>
) -> Result<HttpResponse, UserTransactionError> {
    let user = user.into_inner();
    let pagination_request = PaginationRequest {
        cursor: query.cursor.clone(),
        limit: query.limit,
    };

    let (mut transactions, pagination) = services.user_transaction_service.clone().get_successful_transactions_for_user_with_detail(
        &user,
        &pagination_request
    ).await?;
    let transactions= transactions
        .into_iter()
        .map(|txn| txn.into())
        .collect();

    Ok(HttpResponse::Ok().json(TransactionsForUserResponse {
        transactions: transactions,
        pagination: pagination
    }))
}

#[get("/card/{public_id}/")]
async fn get_transactions_for_card(
    user: web::ReqData<UserModel>,
    public_id: web::Path<Uuid>,
    query: web::Query<CursorQueryParams>,
    services: web::Data<Services>
) -> Result<HttpResponse, UserTransactionError> {
    let user = user.into_inner();

    let pagination_request = PaginationRequest {
        cursor: query.cursor.clone(),
        limit: query.limit,
    };

    let (mut transactions, pagination) = services.user_transaction_service.clone().get_successful_transactions_for_user_and_card_with_detail(
        &user,
        &public_id,
        &pagination_request
    ).await?;
    let transactions = transactions
        .into_iter()
        .map(|card| card.into())
        .collect();
    Ok(HttpResponse::Ok().json( TransactionsForWalletCardResponse {
        transactions: transactions,
        pagination: pagination
    }))
}