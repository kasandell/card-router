#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use lithic_client::models::card::State;
    use crate::error_type::ErrorType;
    use crate::passthrough_card::constant::PassthroughCardStatus;
    use crate::test_helper::user::create_user;
    use crate::passthrough_card::entity::PassthroughCard;
    use crate::lithic_service::service::MockLithicServiceTrait;
    use crate::passthrough_card::engine::Engine;
    use crate::test_helper::passthrough_card::create_mock_lithic_card_for_status_update;

    #[actix_web::test]
    async fn test_create_card_for_user() {
        crate::test_helper::general::init();
        let mut lithic_service = MockLithicServiceTrait::new();
        let user = create_user().await;
        let exp_month = "09";
        let exp_year = "2026";
        let pin = "1234";
        let mut card = create_mock_lithic_card_for_status_update();
        card.exp_month = Some(exp_month.to_string());
        card.exp_year = Some(exp_year.to_string());
        let mut lithic_return = card.clone();
        lithic_return.state = State::Open;
        lithic_service.expect_create_card()
            .times(1)
            .return_const(
                Ok(lithic_return)
            );

        let engine = Arc::new(Engine::new_with_service(Arc::new(lithic_service)));
        let ret = engine.clone().issue_card_to_user(
            &user,
            pin
        ).await.expect("should create");

        assert_eq!(ret.token, card.token.to_string());
        assert_eq!(String::from(&PassthroughCardStatus::OPEN), ret.passthrough_card_status);
        ret.delete_self().await.expect("Should delete");
        user.delete_self().await.expect("Should delete");
    }

    #[actix_web::test]
    async fn test_create_card_for_user_fails_on_dupe() {
        crate::test_helper::general::init();
        let mut lithic_service = MockLithicServiceTrait::new();
        let user = create_user().await;
        let exp_month = "09";
        let exp_year = "2026";
        let pin = "1234";
        let mut card = create_mock_lithic_card_for_status_update();
        card.exp_month = Some(exp_month.to_string());
        card.exp_year = Some(exp_year.to_string());
        let mut lithic_return = card.clone();
        lithic_return.state = State::Open;
        lithic_service.expect_create_card()
            .times(1) //TODO: revert back to 1
            .return_const(
                Ok(lithic_return)
            );

        let engine = Arc::new(Engine::new_with_service(Arc::new(lithic_service)));
        let ret = engine.clone().issue_card_to_user(
            &user,
            pin
        ).await.expect("should create");

        assert_eq!(ret.token, card.token.to_string());
        assert_eq!(String::from(&PassthroughCardStatus::OPEN), ret.passthrough_card_status);
        let error = engine.clone().issue_card_to_user(
            &user,
            pin
        ).await.expect_err("This should throw an error");
        assert_eq!(ErrorType::Conflict, error.error_type);
        ret.delete_self().await.expect("Should delete");
        user.delete_self().await.expect("Should delete");
    }

    #[actix_web::test]
    async fn test_status_successfully_pauses() {
        crate::test_helper::general::init();
        let mut lithic_service = MockLithicServiceTrait::new();
        let user = create_user().await;
        let exp_month = "09";
        let exp_year = "2026";
        let mut card = create_mock_lithic_card_for_status_update();
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
        ).await.expect("Card should create");
        let engine = Arc::new(Engine::new_with_service(Arc::new(lithic_service)));
        let res = engine.clone().update_card_status(
            &user,
            PassthroughCardStatus::PAUSED
        ).await;

        assert!(res.is_ok());
        created_card = PassthroughCard::get(created_card.id).await.expect("card should be found");
        assert_eq!(
            String::from(&PassthroughCardStatus::PAUSED),
            created_card.passthrough_card_status
        );

        created_card.delete_self().await.expect("card should delete cleanly");
        user.delete_self().await.expect("User should delete");
    }

    #[actix_web::test]
    async fn test_status_successfully_closes() {
        crate::test_helper::general::init();
        let mut lithic_service = MockLithicServiceTrait::new();
        let user = create_user().await;
        let exp_month = "09";
        let exp_year = "2026";
        let mut card = create_mock_lithic_card_for_status_update();
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
        ).await.expect("Card should create");
        let engine = Arc::new(Engine::new_with_service(Arc::new(lithic_service)));
        let res = engine.clone().update_card_status(
            &user,
            PassthroughCardStatus::CLOSED
        ).await;

        assert!(res.is_ok());
        created_card = PassthroughCard::get(created_card.id).await.expect("card should be found");
        assert_eq!(
            String::from(&PassthroughCardStatus::CLOSED),
            created_card.passthrough_card_status
        );

        created_card.delete_self().await.expect("card should delete cleanly");
        user.delete_self().await.expect("User should delete");
    }

    #[actix_web::test]
    async fn test_status_fails_to_reopen() {
        crate::test_helper::general::init();
        let mut lithic_service = MockLithicServiceTrait::new();
        let user = create_user().await;
        let exp_month = "09";
        let exp_year = "2026";
        let mut card = create_mock_lithic_card_for_status_update();
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
        ).await.expect("Card should create");
        let engine = Arc::new(Engine::new_with_service(Arc::new(lithic_service)));
        let res = engine.clone().update_card_status(
            &user,
            PassthroughCardStatus::CLOSED
        ).await;

        assert!(res.is_ok());
        created_card = PassthroughCard::get(created_card.id).await.expect("card should be found");
        assert_eq!(
            String::from(&PassthroughCardStatus::CLOSED),
            created_card.passthrough_card_status
        );

        let res = engine.clone().update_card_status(
            &user,
            PassthroughCardStatus::OPEN
        ).await.expect_err("Should throw api errror");

        assert_eq!(ErrorType::NotFound, res.error_type);

        created_card = PassthroughCard::get(created_card.id).await.expect("card should be found");
        assert_eq!(
            String::from(&PassthroughCardStatus::CLOSED),
            created_card.passthrough_card_status
        );


        created_card.delete_self().await.expect("card should delete cleanly");
        user.delete_self().await.expect("User should delete");
    }
}