#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use chrono::NaiveDate;
    use lithic_client::models::{FundingAccount};
    use lithic_client::models::card::{SpendLimitDuration, State, Type, Card};
    use lithic_client::models::funding_account::{State as FundingState, Type as FundingType};
    use uuid::Uuid;
    use crate::passthrough_card::constant::PassthroughCardStatus;
    use crate::test_helper::initialize_user;
    use crate::passthrough_card::entity::{
        LithicCard,
        PassthroughCard,
        PassthroughCardStatusUpdate,
        InsertablePassthroughCard
    };
    use crate::lithic_service::service::MockLithicServiceTrait;
    use crate::passthrough_card::engine::Engine;
    use crate::user::entity::{User, UserMessage};

    #[actix_web::test]
    async fn test_create_card_for_user() {
        crate::test::init();
        let mut lithic_service = MockLithicServiceTrait::new();
        let user = initialize_user();
        let exp_month = "09";
        let exp_year = "2026";
        let pin = "1234";
        let mut card = create_test_card();
        card.exp_month = Some(exp_month.to_string());
        card.exp_year = Some(exp_year.to_string());
        let mut lithic_return = card.clone();
        lithic_return.state = State::Open;
        lithic_service.expect_create_card()
            .times(1)
            .return_const(
                Ok(lithic_return)
            );

        let engine = Engine::new_with_service(Box::new(lithic_service));
        let ret = engine.issue_card_to_user(
            &user,
            pin.to_string()
        ).await.expect("should create");

        assert_eq!(ret.token, card.token.to_string());
        assert_eq!(String::from(&PassthroughCardStatus::OPEN), ret.passthrough_card_status);
        ret.delete_self().expect("Should delete");
        user.delete_self().expect("Should delete");
    }

    #[actix_web::test]
    async fn test_create_card_for_user_fails_on_dupe() {
        crate::test::init();
        let mut lithic_service = MockLithicServiceTrait::new();
        let user = initialize_user();
        let exp_month = "09";
        let exp_year = "2026";
        let pin = "1234";
        let mut card = create_test_card();
        card.exp_month = Some(exp_month.to_string());
        card.exp_year = Some(exp_year.to_string());
        let mut lithic_return = card.clone();
        lithic_return.state = State::Open;
        lithic_service.expect_create_card()
            .times(1) //TODO: revert back to 1
            .return_const(
                Ok(lithic_return)
            );

        let engine = Engine::new_with_service(Box::new(lithic_service));
        let ret = engine.issue_card_to_user(
            &user,
            pin.to_string()
        ).await.expect("should create");

        assert_eq!(ret.token, card.token.to_string());
        assert_eq!(String::from(&PassthroughCardStatus::OPEN), ret.passthrough_card_status);
        let error = engine.issue_card_to_user(
            &user,
            pin.to_string()
        ).await.expect_err("This should throw an error");
        assert_eq!(409, error.status_code);
        ret.delete_self().expect("Should delete");
        user.delete_self().expect("Should delete");
    }

    #[actix_web::test]
    async fn test_status_successfully_pauses() {
        crate::test::init();
        let mut lithic_service = MockLithicServiceTrait::new();
        let user = initialize_user();
        let exp_month = "09";
        let exp_year = "2026";
        let mut card = create_test_card();
        card.exp_month = Some(exp_month.to_string());
        card.exp_year = Some(exp_year.to_string());
        let mut lithic_return = card.clone();
        lithic_return.state = State::Paused;
        lithic_service.expect_pause_card()
            .times(1) //TODO: revert back to 1
            .return_const(
                Ok(lithic_return)
            );
        let mut created_card = PassthroughCard::create_from_api_card(
            &card,
            &user
        ).expect("Card should create");
        let engine = Engine::new_with_service(Box::new(lithic_service));
        let res = engine.update_card_status(
            &user,
            PassthroughCardStatus::PAUSED
        ).await;

        assert!(res.is_ok());
        created_card = PassthroughCard::get(created_card.id).expect("card should be found");
        assert_eq!(
            String::from(&PassthroughCardStatus::PAUSED),
            created_card.passthrough_card_status
        );

        created_card.delete_self().expect("card should delete cleanly");
        user.delete_self().expect("User should delete");
    }

    #[actix_web::test]
    async fn test_status_successfully_closes() {
        crate::test::init();
        let mut lithic_service = MockLithicServiceTrait::new();
        let user = initialize_user();
        let exp_month = "09";
        let exp_year = "2026";
        let mut card = create_test_card();
        card.exp_month = Some(exp_month.to_string());
        card.exp_year = Some(exp_year.to_string());
        let mut lithic_return = card.clone();
        lithic_return.state = State::Closed;
        lithic_service.expect_close_card()
            .times(1) // TODO: bring back to 1 once we have the code working in prod with lithic key
            .return_const(
                Ok(lithic_return)
            );
        let mut created_card = PassthroughCard::create_from_api_card(
            &card,
            &user
        ).expect("Card should create");
        let engine = Engine::new_with_service(Box::new(lithic_service));
        let res = engine.update_card_status(
            &user,
            PassthroughCardStatus::CLOSED
        ).await;

        assert!(res.is_ok());
        created_card = PassthroughCard::get(created_card.id).expect("card should be found");
        assert_eq!(
            String::from(&PassthroughCardStatus::CLOSED),
            created_card.passthrough_card_status
        );

        created_card.delete_self().expect("card should delete cleanly");
        user.delete_self().expect("User should delete");
    }

    #[actix_web::test]
    async fn test_status_fails_to_reopen() {
        crate::test::init();
        let mut lithic_service = MockLithicServiceTrait::new();
        let user = initialize_user();
        let exp_month = "09";
        let exp_year = "2026";
        let mut card = create_test_card();
        card.exp_month = Some(exp_month.to_string());
        card.exp_year = Some(exp_year.to_string());
        let mut lithic_return = card.clone();
        lithic_return.state = State::Closed;
        lithic_service.expect_close_card()
            .times(1) //TODO: revert back to 1
            .return_const(
                Ok(lithic_return)
            );

        let mut created_card = PassthroughCard::create_from_api_card(
            &card,
            &user
        ).expect("Card should create");
        let engine = Engine::new_with_service(Box::new(lithic_service));
        let res = engine.update_card_status(
            &user,
            PassthroughCardStatus::CLOSED
        ).await;

        assert!(res.is_ok());
        created_card = PassthroughCard::get(created_card.id).expect("card should be found");
        assert_eq!(
            String::from(&PassthroughCardStatus::CLOSED),
            created_card.passthrough_card_status
        );

        let res = engine.update_card_status(
            &user,
            PassthroughCardStatus::OPEN
        ).await.expect_err("Should throw api errror");

        assert_eq!(404, res.status_code);

        created_card = PassthroughCard::get(created_card.id).expect("card should be found");
        assert_eq!(
            String::from(&PassthroughCardStatus::CLOSED),
            created_card.passthrough_card_status
        );


        created_card.delete_self().expect("card should delete cleanly");
        user.delete_self().expect("User should delete");
    }


    fn create_test_card() -> Card {
        Card::new(
            "".to_string(),
            FundingAccount::new(
                "".to_string(),
                "1234".to_string(),
                FundingState::Enabled,
                Uuid::new_v4(),
                FundingType::Checking
            ),
            "1234".to_string(),
            500,
            SpendLimitDuration::Forever,
            State::Open,
            Uuid::new_v4(),
            Type::Virtual
        )
    }
}