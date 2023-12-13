use adyen_webhooks::models::{
    RecurringContractNotificationRequest,
    RecurringContractNotificationRequestItemWrapper,
    recurring_contract_notification_request_item::EventCode,
};
use lazy_static::lazy_static;

use crate::wallet::entity::{
    WalletCardAttempt,
    UpdateCardAttempt,
    NewCard,
    Wallet
};

use crate::wallet::constant::WalletCardAttemptStatus;
use crate::api_error::ApiError;
use crate::charge_engine::engine::Engine as ChargeEngine;
use crate::asa_request::entity::AsaRequest;
use crate::rule_engine::engine::RuleEngine;
use crate::user::entity::User;

pub struct LithicHandler {}

lazy_static! {
    static ref CHARGE_ENGINE: ChargeEngine = ChargeEngine::new();
}

impl LithicHandler {
    pub fn handle(request: AsaRequest) -> Result<(), ApiError>{
        // TODO: do a reverse lookup based on the card token to get the user
        let user = User {
            id: 1,
            public_id: Default::default(),
            email: "".to_string(),
            password: "".to_string(),
            created_at: Default::default(),
            updated_at: Default::default(),
        };
        let cards = RuleEngine::order_user_cards_for_request(
            request.clone(),
            &user
        )?;

        CHARGE_ENGINE.charge_wallet(
            &user,
            &cards,
            request.amount,
            &request.merchant.mcc,
            &request.merchant.descriptor
        )
    }
}