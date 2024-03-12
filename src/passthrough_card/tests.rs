#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use lithic_client::models::card::State;
    use mockall::Sequence;
    use crate::error::error_type::ErrorType;
    use crate::passthrough_card::constant::PassthroughCardStatus;
    use crate::test_helper::user::{create_mock_user, create_user};
    use crate::passthrough_card::entity::PassthroughCard;
    use crate::lithic::service::MockLithicServiceTrait;
    use crate::passthrough_card::dao::MockPassthroughCardDaoTrait;
    use crate::passthrough_card::service::PassthroughCardService;
    use crate::test_helper::passthrough_card::{create_mock_lithic_card, create_mock_lithic_card_for_status_update, create_mock_passthrough_card};

    #[actix_web::test]
    async fn test_create_card_for_user() {
        let mut lithic_service = MockLithicServiceTrait::new();
        let mut dao_mock = MockPassthroughCardDaoTrait::new();
        let user = create_mock_user();
        let exp_month = "09";
        let exp_year = "2026";
        let pin = "1234";
        let mut lithic_card = create_mock_lithic_card();
        let mut passthrough_card = create_mock_passthrough_card();
        passthrough_card.token = lithic_card.token.to_string();
        lithic_card.exp_month = Some(exp_month.to_string());
        lithic_card.exp_year = Some(exp_year.to_string());
        let mut lithic_return = lithic_card.clone();
        lithic_return.state = State::Open;
        lithic_service
            .expect_create_card()
            .times(1)
            .return_const(Ok(lithic_return));
        dao_mock
            .expect_create_from_api_card()
            .times(1)
            .return_const(Ok(passthrough_card.clone()));

        dao_mock
            .expect_find_cards_for_user()
            .times(1)
            .return_const(Ok(vec![]));

        let engine = Arc::new(PassthroughCardService::new_with_services(
            Arc::new(lithic_service),
            Arc::new(dao_mock),
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

    #[actix_web::test]
    async fn test_create_card_for_user_fails_on_dupe() {
        let mut lithic_service = MockLithicServiceTrait::new();
        let mut dao_mock = MockPassthroughCardDaoTrait::new();
        let user = create_mock_user();
        let exp_month = "09";
        let exp_year = "2026";
        let pin = "1234";
        let mut lithic_card = create_mock_lithic_card();
        let mut passthrough_card = create_mock_passthrough_card();
        passthrough_card.token = lithic_card.token.to_string();
        lithic_card.exp_month = Some(exp_month.to_string());
        lithic_card.exp_year = Some(exp_year.to_string());
        let mut lithic_return = lithic_card.clone();
        lithic_return.state = State::Open;
        lithic_service
            .expect_create_card()
            .times(1)
            .return_const(Ok(lithic_return));

        let mut seq = Sequence::new();
        dao_mock
            .expect_create_from_api_card()
            .times(1)
            .return_const(Ok(passthrough_card.clone()));

        dao_mock
            .expect_find_cards_for_user()
            .once()
            .in_sequence(&mut seq)
            .return_const(Ok(vec![]));
        dao_mock
            .expect_find_cards_for_user()
            .once()
            .in_sequence(&mut seq)
            .return_const(Ok(vec![passthrough_card.clone()]));

        let engine = Arc::new(PassthroughCardService::new_with_services(
            Arc::new(lithic_service),
            Arc::new(dao_mock),
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
        assert_eq!(ErrorType::Conflict, error.error_type);
    }

    #[actix_web::test]
    async fn test_status_successfully_pauses() {
        let mut lithic_service = MockLithicServiceTrait::new();
        let mut dao_mock = MockPassthroughCardDaoTrait::new();
        let user = create_mock_user();
        let exp_month = "09";
        let exp_year = "2026";
        let mut card = create_mock_lithic_card();
        card.state = State::Paused;
        card.exp_month = Some(exp_month.to_string());
        card.exp_year = Some(exp_year.to_string());
        let mut created_card = create_mock_passthrough_card();
        created_card.token = card.token.to_string();
        let mut paused_card = created_card.clone();
        paused_card.passthrough_card_status = PassthroughCardStatus::Paused;
        let mut lithic_return = card.clone();
        lithic_return.state = State::Paused;
        lithic_service
            .expect_pause_card()
            .times(1) //TODO: revert back to 1
            .return_const(Ok(lithic_return));
        dao_mock
            .expect_update_status()
            .times(1)
            .return_const(Ok(paused_card.clone()));
        dao_mock
            .expect_find_cards_for_user()
            .times(1)
            .return_const(Ok(vec![created_card.clone()]));
        let engine = Arc::new(PassthroughCardService::new_with_services(
            Arc::new(lithic_service),
            Arc::new(dao_mock),
        ));
        let res = engine
            .clone()
            .update_card_status(&user, PassthroughCardStatus::Paused)
            .await;

        assert!(res.is_ok());
    }

    #[actix_web::test]
    async fn test_status_successfully_closes() {
        let mut lithic_service = MockLithicServiceTrait::new();
        let mut mock_dao = MockPassthroughCardDaoTrait::new();
        let user = create_mock_user();
        let exp_month = "09";
        let exp_year = "2026";
        let mut card = create_mock_lithic_card();
        card.exp_month = Some(exp_month.to_string());
        card.exp_year = Some(exp_year.to_string());
        let mut lithic_return = card.clone();
        lithic_return.state = State::Closed;
        lithic_service
            .expect_close_card()
            .times(1) // TODO: bring back to 1 once we have the code working in prod with lithic key
            .return_const(Ok(lithic_return));
        let mut created_card = create_mock_passthrough_card();
        let mut closed_card = created_card.clone();
        closed_card.passthrough_card_status = PassthroughCardStatus::Closed;
        mock_dao
            .expect_find_cards_for_user()
            .times(1)
            .return_const(Ok(vec![created_card.clone()]));
        mock_dao
            .expect_update_status()
            .times(1)
            .return_const(Ok(closed_card));
        let engine = Arc::new(PassthroughCardService::new_with_services(
            Arc::new(lithic_service),
            Arc::new(mock_dao),
        ));

        let res = engine
            .clone()
            .update_card_status(&user, PassthroughCardStatus::Closed)
            .await;

        assert!(res.is_ok());
    }

    #[actix_web::test]
    async fn test_status_fails_to_reopen() {
        let mut lithic_service = MockLithicServiceTrait::new();
        let mut mock_dao = MockPassthroughCardDaoTrait::new();
        let user = create_mock_user();
        let exp_month = "09";
        let exp_year = "2026";
        let mut card = create_mock_lithic_card();
        card.exp_month = Some(exp_month.to_string());
        card.exp_year = Some(exp_year.to_string());
        let mut lithic_return = card.clone();
        lithic_return.state = State::Closed;
        lithic_service
            .expect_close_card()
            .times(1) //TODO: revert back to 1
            .return_const(Ok(lithic_return));

        let mut created_card = create_mock_passthrough_card();
        let mut closed_card = created_card.clone();
        closed_card.passthrough_card_status = PassthroughCardStatus::Closed;
        let mut seq = Sequence::new();
        mock_dao
            .expect_find_cards_for_user()
            .once()
            .in_sequence(&mut seq)
            .return_const(Ok(vec![created_card.clone()]));
        mock_dao
            .expect_find_cards_for_user()
            .once()
            .in_sequence(&mut seq)
            .return_const(Ok(vec![closed_card.clone()]));

        mock_dao
            .expect_update_status()
            .times(1)
            .return_const(Ok(closed_card.clone()));

        let engine = Arc::new(PassthroughCardService::new_with_services(
            Arc::new(lithic_service),
            Arc::new(mock_dao),
        ));
        let res = engine
            .clone()
            .update_card_status(&user, PassthroughCardStatus::Closed)
            .await;

        assert!(res.is_ok());

        let res = engine
            .clone()
            .update_card_status(&user, PassthroughCardStatus::Open)
            .await
            .expect_err("Should throw api error");

        assert_eq!(ErrorType::NotFound, res.error_type);
    }
}