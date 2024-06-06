use std::sync::Arc;
use chrono::Utc;
use footprint::models::CreateClientTokenResponse;
use mockall::predicate::eq;
use uuid::Uuid;
use crate::credit_card_type::service::{CreditCardService, CreditCardServiceTrait};
use crate::footprint::service::MockFootprintServiceTrait;
use crate::test_helper::credit_card::create_mock_credit_card;
use crate::user::model::UserModel;
use crate::wallet::constant::{WalletCardAttemptStatus, WalletStatus};
use crate::wallet::model::{WalletModel as Wallet, WalletCardAttemptModel as WalletCardAttempt, WalletModelWithRule, WalletModel};
use crate::wallet::request::{MatchRequest, RegisterAttemptRequest};
use crate::wallet::response::WalletCardAttemptResponse;
use crate::wallet::service::{WalletService, WalletServiceTrait};

pub fn create_mock_wallet() -> Wallet {
    Wallet {
        id: 1,
        public_id: Default::default(),
        user_id: 1,
        payment_method_id: "".to_string(),
        created_at: Default::default(),
        credit_card_id: 0,
        wallet_card_attempt_id: 0,
        status: WalletStatus::Active,
    }
}

pub fn create_mock_wallet_with_rule() -> WalletModelWithRule {
    WalletModelWithRule {
        id: -1,
        public_id: Default::default(),
        user_id: 1,
        payment_method_id: "".to_string(),
        created_at: Default::default(),
        credit_card_id: 0,
        wallet_card_attempt_id: 0,
        status: WalletStatus::Active,
        rule_id: Some(1),
    }
}

pub fn create_mock_wallet_with_args(
    id: i32,
    user_id: i32,
    credit_card_id: i32
) -> Wallet {
    Wallet {
        id: id,
        public_id: Uuid::new_v4(),
        user_id: user_id,
        payment_method_id: Uuid::new_v4().to_string(),
        created_at: Utc::now().naive_utc(),
        credit_card_id: credit_card_id,
        wallet_card_attempt_id: 0,
        status: WalletStatus::Active,
    }
}

pub fn create_mock_wallet_attempt() -> WalletCardAttempt {
    WalletCardAttempt {
        id: 1,
        public_id: Default::default(),
        user_id: 1,
        credit_card_id: 1,
        expected_reference_id: "".to_string(),
        status: WalletCardAttemptStatus::Pending,
        created_at: Default::default(),
    }
}

pub async fn create_wallet(user: &UserModel) -> Wallet {
    let mut footprint_service = MockFootprintServiceTrait::new();
    let mut credit_card_service = Arc::new(CreditCardService::new());
    let card = credit_card_service.clone().list_all_card_types()
        .await.expect("cards").get(0).expect("gets card").clone();

    footprint_service.expect_create_client_token()
        .once()
        .return_once(move |_, _|
            Ok(CreateClientTokenResponse {
                expires_at: None,
                token: "12345".to_string(),
            })
        );

    let svc = Arc::new(WalletService::new_with_services(
        credit_card_service.clone(),
        Arc::new(footprint_service),
    ));
    let att = svc.clone().register_new_attempt(
        &user,
        &RegisterAttemptRequest {
            credit_card_type_public_id: card.public_id
        }
    ).await.expect("should create attempt");
    
    let card = svc.clone().match_card(
        &user,
        &MatchRequest {
            reference_id: att.reference_id.clone(),
        }
    ).await.expect("should create card");

    card
}

pub async fn create_wallet_with_rule(user: &UserModel) -> WalletModelWithRule {
    let card = create_wallet(user).await;
    card.into()
}