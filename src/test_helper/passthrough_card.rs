use std::sync::Arc;
use chrono::NaiveDate;
use lithic_client::models::{
    Card, FundingAccount,
    card::Card as InnerCard,
    card::{SpendLimitDuration, State, Type},
    funding_account::{State as FundingState, Type as FundingType}
};
use uuid::Uuid;
use crate::lithic::service::MockLithicServiceTrait;
use crate::passthrough_card::constant::{PassthroughCardStatus, PassthroughCardType};
use crate::passthrough_card::model::{PassthroughCardModel as PassthroughCard, PassthroughCardModel};
use crate::passthrough_card::service::{PassthroughCardService, PassthroughCardServiceTrait};
use crate::test_helper::constant::{EXP_MONTH, EXP_YEAR, LAST_FOUR};
use crate::user::model::UserModel;

pub fn create_mock_passthrough_card() -> PassthroughCard {
    PassthroughCard {
        id: 0,
        public_id: Default::default(),
        passthrough_card_status: PassthroughCardStatus::Open,
        is_active: Some(true),
        user_id: 1,
        token: "".to_string(),
        expiration: NaiveDate::MAX,
        last_four: "1234".to_string(),
        passthrough_card_type: PassthroughCardType::Virtual,
        created_at: Default::default(),
    }
}



pub fn create_mock_lithic_card_for_status_update() -> InnerCard {
    Card::new(
        "".to_string(),
        FundingAccount::new(
            "".to_string(),
            "1234".to_string(),
            FundingState::Enabled,
            Uuid::new_v4(),
            FundingType::Checking
        ),
        LAST_FOUR.to_string(),
        500,
        SpendLimitDuration::Forever,
        State::Open,
        Uuid::new_v4(),
        Type::Virtual
    )
}

pub fn create_mock_lithic_card() -> Card {
    Card {
        created: "".to_string(),
        cvv: None,
        funding: Box::new(FundingAccount {
            account_name: None,
            created: "".to_string(),
            last_four: "".to_string(),
            nickname: None,
            state: Default::default(),
            token: Default::default(),
            r#type: Default::default(),
        }),
        exp_month: Some(EXP_MONTH.to_string()),
        exp_year: Some(EXP_YEAR.to_string()),
        hostname: None,
        last_four: LAST_FOUR.to_string(),
        memo: None,
        pan: None,
        spend_limit: 0,
        spend_limit_duration: Default::default(),
        state: Default::default(),
        auth_rule_tokens: None,
        token: Default::default(),
        r#type: Default::default(),
        digital_card_art_token: None,
    }
}

pub async fn create_passthrough_card(user: &UserModel) -> PassthroughCardModel {
    let mut mock_lithic = MockLithicServiceTrait::new();
    mock_lithic.expect_create_card()
        .times(1)
        .return_once(move |_, _| Ok(Card {
            created: "".to_string(),
            cvv: None,
            funding: Box::new(FundingAccount {
                account_name: None,
                created: "".to_string(),
                last_four: "".to_string(),
                nickname: None,
                state: Default::default(),
                token: Default::default(),
                r#type: Default::default(),
            }),
            exp_month: Some("09".into()),
            exp_year: Some("2033".into()),
            hostname: None,
            last_four: "1234".to_string(),
            memo: None,
            pan: None,
            spend_limit: 0,
            spend_limit_duration: Default::default(),
            state: Default::default(),
            auth_rule_tokens: None,
            token: Default::default(),
            r#type: Default::default(),
            digital_card_art_token: None,
        }));
    let svc = Arc::new(PassthroughCardService::new_with_services(
        Arc::new(mock_lithic)
    ));
    svc.issue_card_to_user(&user, "1234").await.expect("should issue card")
}
