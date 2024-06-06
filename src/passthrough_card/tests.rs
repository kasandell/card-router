#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use actix_web::test;
    use lithic_client::models::card::State;
    use mockall::Sequence;
    use uuid::Uuid;
    use crate::asa::request::create_example_asa;
    use crate::error::data_error::DataError;
    use crate::lithic::error::LithicError;
    use crate::passthrough_card::constant::PassthroughCardStatus;
    use crate::test_helper::user::{create_mock_user, create_user};
    use crate::passthrough_card::entity::PassthroughCard;
    use crate::lithic::service::MockLithicServiceTrait;
    use crate::passthrough_card::service::{PassthroughCardService, PassthroughCardServiceTrait};
    use crate::test_helper::passthrough_card::{create_mock_lithic_card, create_mock_lithic_card_for_status_update, create_mock_passthrough_card};
    use crate::passthrough_card::error::PassthroughCardError;

    #[test]
    async fn test_create_card_for_user() {
        crate::test_helper::general::init();
        let mut lithic_service = MockLithicServiceTrait::new();
        let user = create_user().await;
        let exp_month = "09";
        let exp_year = "2026";
        let pin = "1234";
        let mut lithic_card = create_mock_lithic_card();
        lithic_card.exp_month = Some(exp_month.to_string());
        lithic_card.exp_year = Some(exp_year.to_string());
        let mut lithic_return = lithic_card.clone();
        lithic_return.state = State::Open;
        lithic_service
            .expect_create_card()
            .times(1)
            .return_once(move |_, _| Ok(lithic_return));

        let engine = Arc::new(PassthroughCardService::new_with_services(
            Arc::new(lithic_service),
        ));
        let ret = engine
            .clone()
            .issue_card_to_user(&user, pin)
            .await
            .expect("should create");

        assert_eq!(ret.token, lithic_card.token.to_string());
        assert_eq!(
            PassthroughCardStatus::Open,
            ret.passthrough_card_status
        );
    }


    #[test]
    async fn test_find_by_token() {
        crate::test_helper::general::init();
        let mut lithic_service = MockLithicServiceTrait::new();
        let user = create_user().await;
        let exp_month = "09";
        let exp_year = "2026";
        let pin = "1234";
        let mut lithic_card = create_mock_lithic_card();
        lithic_card.token = Uuid::new_v4();
        lithic_card.exp_month = Some(exp_month.to_string());
        lithic_card.exp_year = Some(exp_year.to_string());
        let mut lithic_return = lithic_card.clone();
        lithic_return.state = State::Open;
        lithic_service
            .expect_create_card()
            .times(1)
            .return_once(move |_, _| Ok(lithic_return));

        let engine = Arc::new(PassthroughCardService::new_with_services(
            Arc::new(lithic_service),
        ));
        let ret = engine
            .clone()
            .issue_card_to_user(&user, pin)
            .await
            .expect("should create");

        assert_eq!(ret.token, lithic_card.token.to_string());
        assert_eq!(
            PassthroughCardStatus::Open,
            ret.passthrough_card_status
        );

        let card = engine
            .clone()
            .get_by_token(&ret.token)
            .await
            .expect("should find");

        assert_eq!(card.id, ret.id);
        assert_eq!(card.passthrough_card_status, PassthroughCardStatus::Open);
    }


    #[test]
    async fn test_find_by_fails() {
        crate::test_helper::general::init();
        let mut lithic_service = MockLithicServiceTrait::new();
        let user = create_user().await;

        let engine = Arc::new(PassthroughCardService::new_with_services(
            Arc::new(lithic_service),
        ));

        let not_found = engine
            .clone()
            .get_by_token(&Uuid::new_v4().to_string())
            .await
            .expect_err("should not find");

        assert_eq!(PassthroughCardError::CardNotFound("test".into()), not_found);
    }

    #[test]
    async fn test_create_card_for_user_fails_on_dupe() {
        crate::test_helper::general::init();
        let mut lithic_service = MockLithicServiceTrait::new();
        let user = create_user().await;
        let exp_month = "09";
        let exp_year = "2026";
        let pin = "1234";
        let mut lithic_card = create_mock_lithic_card();
        lithic_card.exp_month = Some(exp_month.to_string());
        lithic_card.exp_year = Some(exp_year.to_string());
        let mut lithic_return = lithic_card.clone();
        lithic_return.state = State::Open;
        lithic_service
            .expect_create_card()
            .times(1)
            .return_once(move |_, _| Ok(lithic_return));

        let engine = Arc::new(PassthroughCardService::new_with_services(
            Arc::new(lithic_service),
        ));
        let ret = engine
            .clone()
            .issue_card_to_user(&user, pin)
            .await
            .expect("should create");

        assert_eq!(ret.token, lithic_card.token.to_string());
        assert_eq!(
            PassthroughCardStatus::Open,
            ret.passthrough_card_status
        );
        let error = engine
            .clone()
            .issue_card_to_user(&user, pin)
            .await
            .expect_err("This should throw an error");
        assert_eq!(PassthroughCardError::ActiveCardExists("Test".into()), error);
    }

    #[test]
    async fn test_status_successfully_pauses() {
        crate::test_helper::general::init();
        let mut lithic_service = MockLithicServiceTrait::new();
        let user = create_user().await;
        let exp_month = "09";
        let exp_year = "2026";
        let mut card = create_mock_lithic_card();
        card.exp_month = Some(exp_month.to_string());
        card.exp_year = Some(exp_year.to_string());
        let lithic_return = card.clone();
        let mut lithic_pause = card.clone();
        lithic_pause.state = State::Paused;
        lithic_service
            .expect_create_card()
            .times(1)
            .return_once(move |_, _| Ok(lithic_return));

        lithic_service
            .expect_pause_card()
            .times(1)
            .return_once(move |_| Ok(lithic_pause));

        let engine = Arc::new(PassthroughCardService::new_with_services(
            Arc::new(lithic_service),
        ));
        let created_card = engine
            .clone()
            .issue_card_to_user(&user, "1234").await.expect("ok");
        let res = engine
            .clone()
            .update_card_status(&user, PassthroughCardStatus::Paused)
            .await;

        assert!(res.is_ok());
        let pc_db = PassthroughCard::get_by_token(
            &created_card.token
        ).await.expect("card is found");
        assert!(pc_db.is_active.unwrap());
        assert_eq!(pc_db.passthrough_card_status, PassthroughCardStatus::Paused);
    }

    #[test]
    async fn test_status_successfully_closes() {
        crate::test_helper::general::init();
        let mut lithic_service = MockLithicServiceTrait::new();
        let user = create_user().await;
        let exp_month = "09";
        let exp_year = "2026";
        let mut card = create_mock_lithic_card();
        card.exp_month = Some(exp_month.to_string());
        card.exp_year = Some(exp_year.to_string());
        let lithic_return = card.clone();
        let mut lithic_close = card.clone();
        lithic_close.state = State::Closed;
        lithic_service
            .expect_create_card()
            .times(1)
            .return_once(move |_, _| Ok(lithic_return));
        lithic_service
            .expect_close_card()
            .times(1) // TODO: bring back to 1 once we have the code working in prod with lithic key
            .return_once(move |_| Ok(lithic_close));

        let engine = Arc::new(PassthroughCardService::new_with_services(
            Arc::new(lithic_service),
        ));

        let created_card = engine
            .clone()
            .issue_card_to_user(&user, "1234")
            .await
            .expect("should create");

        let res = engine
            .clone()
            .update_card_status(&user, PassthroughCardStatus::Closed)
            .await;

        assert!(res.is_ok());
        let pc_db = PassthroughCard::get_by_token(&created_card.token)
            .await
            .expect("should find");

        assert!(pc_db.is_active.is_none());
        assert_eq!(pc_db.passthrough_card_status, PassthroughCardStatus::Closed);
    }

    #[test]
    async fn test_status_fails_to_reopen() {
        crate::test_helper::general::init();
        let mut lithic_service = MockLithicServiceTrait::new();
        let user = create_user().await;
        let exp_month = "09";
        let exp_year = "2026";
        let mut card = create_mock_lithic_card();
        card.exp_month = Some(exp_month.to_string());
        card.exp_year = Some(exp_year.to_string());
        let lithic_create = card.clone();

        let mut lithic_return = card.clone();
        lithic_return.state = State::Closed;

        lithic_service
            .expect_create_card()
            .times(1)
            .return_once(move |_, _| Ok(lithic_create));

        lithic_service
            .expect_close_card()
            .times(1) //TODO: revert back to 1
            .return_once(move |_| Ok(lithic_return));

        let engine = Arc::new(PassthroughCardService::new_with_services(
            Arc::new(lithic_service),
        ));
        let card = engine
            .clone()
            .issue_card_to_user(&user, "1234")
            .await
            .expect("creates");
        let res = engine
            .clone()
            .update_card_status(&user, PassthroughCardStatus::Closed)
            .await;

        assert!(res.is_ok());
        let mut pc_db = PassthroughCard::get_by_token(&card.token)
            .await
            .expect("finds card");
        assert_eq!(pc_db.passthrough_card_status, PassthroughCardStatus::Closed);
        assert!(pc_db.is_active.is_none());

        let res = engine
            .clone()
            .update_card_status(&user, PassthroughCardStatus::Open)
            .await
            .expect_err("Should throw api error");

        assert_eq!(PassthroughCardError::CardNotFound("Cannot update from closed".into()), res);

        let mut pc_db = PassthroughCard::get_by_token(&card.token)
            .await
            .expect("finds card");
        assert_eq!(pc_db.passthrough_card_status, PassthroughCardStatus::Closed);
        assert!(pc_db.is_active.is_none());
    }

    #[test]
    async fn test_status_successfully_reopens() {
        crate::test_helper::general::init();
        let mut lithic_service = MockLithicServiceTrait::new();
        let user = create_user().await;
        let exp_month = "09";
        let exp_year = "2026";
        let mut card = create_mock_lithic_card();
        card.exp_month = Some(exp_month.to_string());
        card.exp_year = Some(exp_year.to_string());
        let lithic_return = card.clone();
        let mut lithic_pause = card.clone();
        lithic_pause.state = State::Paused;
        let lithic_reactivate = card.clone();
        lithic_service
            .expect_create_card()
            .times(1)
            .return_once(move |_, _| Ok(lithic_return));

        lithic_service
            .expect_pause_card()
            .times(1)
            .return_once(move |_| Ok(lithic_pause));

        lithic_service
            .expect_activate_card()
            .times(1)
            .return_once(move |_| Ok(lithic_reactivate));

        let engine = Arc::new(PassthroughCardService::new_with_services(
            Arc::new(lithic_service),
        ));
        let created_card = engine
            .clone()
            .issue_card_to_user(&user, "1234").await.expect("ok");
        let res = engine
            .clone()
            .update_card_status(&user, PassthroughCardStatus::Paused)
            .await;
        assert!(res.is_ok());
        let pc_db = PassthroughCard::get_by_token(
            &created_card.token
        ).await.expect("card is found");
        assert!(pc_db.is_active.unwrap());
        assert_eq!(pc_db.passthrough_card_status, PassthroughCardStatus::Paused);

        let res = engine
            .clone()
            .update_card_status(&user, PassthroughCardStatus::Open)
            .await;
        assert!(res.is_ok());
        let pc_db = PassthroughCard::get_by_token(
            &created_card.token
        ).await.expect("card is found");
        assert!(pc_db.is_active.unwrap());
        assert_eq!(pc_db.passthrough_card_status, PassthroughCardStatus::Open);
    }


    #[test]
    async fn test_status_fails_lithic_rolls_back_db() {
        crate::test_helper::general::init();
        let mut lithic_service = MockLithicServiceTrait::new();
        let user = create_user().await;
        let exp_month = "09";
        let exp_year = "2026";
        let mut card = create_mock_lithic_card();
        card.exp_month = Some(exp_month.to_string());
        card.exp_year = Some(exp_year.to_string());
        let lithic_return = card.clone();
        let mut lithic_pause = card.clone();
        lithic_pause.state = State::Paused;
        let lithic_reactivate = card.clone();
        lithic_service
            .expect_create_card()
            .times(1)
            .return_once(move |_, _| Ok(lithic_return));

        lithic_service
            .expect_pause_card()
            .times(1)
            .return_once(move |_| Ok(lithic_pause));

        lithic_service
            .expect_activate_card()
            .times(1)
            .return_once(move |_| Ok(lithic_reactivate));

        let engine = Arc::new(PassthroughCardService::new_with_services(
            Arc::new(lithic_service),
        ));
        let created_card = engine
            .clone()
            .issue_card_to_user(&user, "1234").await.expect("ok");
        let res = engine
            .clone()
            .update_card_status(&user, PassthroughCardStatus::Paused)
            .await;
        assert!(res.is_ok());
        let pc_db = PassthroughCard::get_by_token(
            &created_card.token
        ).await.expect("card is found");
        assert!(pc_db.is_active.unwrap());
        assert_eq!(pc_db.passthrough_card_status, PassthroughCardStatus::Paused);

        let res = engine
            .clone()
            .update_card_status(&user, PassthroughCardStatus::Open)
            .await;
        assert!(res.is_ok());
        let pc_db = PassthroughCard::get_by_token(
            &created_card.token
        ).await.expect("card is found");
        assert!(pc_db.is_active.unwrap());
        assert_eq!(pc_db.passthrough_card_status, PassthroughCardStatus::Open);
    }


    #[test]
    async fn test_insert_fails_lithic_error() {
        crate::test_helper::general::init();
        let mut lithic_service = MockLithicServiceTrait::new();
        let user = create_user().await;
        lithic_service
            .expect_create_card()
            .times(1)
            .return_once(move |_, _| Err(LithicError::Unexpected("test".into())));

        let svc = Arc::new(PassthroughCardService::new_with_services(
            Arc::new(lithic_service)
        ));
        let error = svc.issue_card_to_user(&user, "1234").await.expect_err("should fail");
        assert_eq!(PassthroughCardError::IssueCard("test".into()), error);
    }

    /*
    #[test]
    async fn test_insert_fails_in_db_rolls_back_lithic() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let mut lithic_service = MockLithicServiceTrait::new();
        let exp_month = "09";
        let exp_year = "2026";
        let mut card = create_mock_lithic_card();
        card.exp_month = Some(exp_month.to_string());
        card.exp_year = Some(exp_year.to_string());
        let lithic_return = card.clone();
        let mut lithic_close = card.clone();
        lithic_close.state = State::Closed;
        lithic_service
            .expect_create_card()
            .times(1)
            .return_once(move |_, _| Ok(lithic_return));

        lithic_service
            .expect_close_card()
            .times(1)
            .return_once(move |_| Ok(lithic_close));

        let mut dao = MockPassthroughCardDaoTrait::new();
        dao.expect_create()
            .times(1)
            .return_once(move |_| Err(DataError::Unexpected("test".into())));

        dao.expect_find_cards_for_user()
            .times(1)
            .return_once(move |_| Ok(vec![]));

        let svc = Arc::new(PassthroughCardService::new_with_mocks(
            Arc::new(lithic_service),
            Arc::new(dao)
        ));
        let error = svc.issue_card_to_user(&user, "1234").await.expect_err("should fail");
        assert_eq!(PassthroughCardError::Unexpected("test".into()), error);
    }

    #[test]
    async fn test_insert_fails_in_db_rolls_back_lithic_fails() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let mut lithic_service = MockLithicServiceTrait::new();
        let exp_month = "09";
        let exp_year = "2026";
        let mut card = create_mock_lithic_card();
        card.exp_month = Some(exp_month.to_string());
        card.exp_year = Some(exp_year.to_string());
        let lithic_return = card.clone();
        let mut lithic_close = card.clone();
        lithic_close.state = State::Closed;
        lithic_service
            .expect_create_card()
            .times(1)
            .return_once(move |_, _| Ok(lithic_return));

        lithic_service
            .expect_close_card()
            .times(1)
            .return_once(move |_| Err(LithicError::Unexpected("test".into())));

        let mut dao = MockPassthroughCardDaoTrait::new();
        dao.expect_create()
            .times(1)
            .return_once(move |_| Err(DataError::Unexpected("test".into())));

        dao.expect_find_cards_for_user()
            .times(1)
            .return_once(move |_| Ok(vec![]));

        let svc = Arc::new(PassthroughCardService::new_with_mocks(
            Arc::new(lithic_service),
            Arc::new(dao)
        ));
        let error = svc.issue_card_to_user(&user, "1234").await.expect_err("should fail");
        assert_eq!(PassthroughCardError::Unexpected("test".into()), error);

    }

     */
}