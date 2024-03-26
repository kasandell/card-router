#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use actix_web::test;
    use chrono::NaiveDate;
    use lithic_client::models::Card;
    use uuid::Uuid;
    use crate::error::data_error::DataError;
    use crate::passthrough_card::constant::{PassthroughCardStatus, PassthroughCardType};
    use crate::passthrough_card::entity::{InsertablePassthroughCard, PassthroughCard};
    use crate::test_helper::passthrough_card::create_mock_lithic_card_with_params;
    use crate::test_helper::user::create_user;
    use crate::util::error::UtilityError::DateError;

    const TOKEN: &str = "12345";
    const TOKEN2: &str = "12346";
    const TOKEN3: &str = "12347";
    const LAST_FOUR: &str = "1234";
    const EXP_MONTH: &str = "09";
    const EXP_YEAR: &str = "2026";

    #[test]
    async fn test_create_insertable_card() {
        let token = Uuid::new_v4();
        crate::test_helper::general::init();
        let user = create_user().await;
        let expected_date = NaiveDate::from_ymd_opt(2026, 9, 1).expect("date should create");
        let card = create_mock_lithic_card_with_params(
            &token,
            &EXP_MONTH,
            &EXP_YEAR,
            &LAST_FOUR
        );
        let insertable_card = InsertablePassthroughCard::try_from((card, &user)).expect("converts");
        assert_eq!(expected_date, insertable_card.expiration);
        assert_eq!(token.to_string(), insertable_card.token);
        assert_eq!(PassthroughCardStatus::Open, insertable_card.passthrough_card_status);
        assert_eq!(PassthroughCardType::Virtual, insertable_card.passthrough_card_type);
        let created_card = PassthroughCard::create(
            insertable_card
        ).await.expect("card should create");

        assert_eq!(user.id, created_card.user_id);
        assert_eq!(expected_date, created_card.expiration);
        assert_eq!(PassthroughCardStatus::Open, created_card.passthrough_card_status);
        assert_eq!(PassthroughCardType::Virtual, created_card.passthrough_card_type);
        assert_eq!(LAST_FOUR, created_card.last_four);
        assert!(created_card.is_active.expect("should be here and true"));
    }
    #[test]
    async fn test_status_update() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let expected_date = NaiveDate::from_ymd_opt(2026, 9, 1).expect("date should create");
        let token = Uuid::new_v4();
        let card = create_mock_lithic_card_with_params(
            &token,
            &EXP_MONTH,
            &EXP_YEAR,
            &LAST_FOUR
        );
        let insertable_card = InsertablePassthroughCard::try_from((card, &user)).expect("converts");
        assert_eq!(expected_date, insertable_card.expiration);
        assert_eq!(token.to_string(), insertable_card.token);
        assert_eq!(PassthroughCardStatus::Open, insertable_card.passthrough_card_status);
        assert_eq!(PassthroughCardType::Virtual, insertable_card.passthrough_card_type);
        let created_card = PassthroughCard::create(
            insertable_card
        ).await.expect("card should create");

        assert_eq!(user.id, created_card.user_id);
        assert_eq!(expected_date, created_card.expiration);
        assert_eq!(PassthroughCardStatus::Open, created_card.passthrough_card_status);
        assert_eq!(PassthroughCardType::Virtual, created_card.passthrough_card_type);
        assert_eq!(LAST_FOUR, created_card.last_four);
        assert!(created_card.is_active.expect("should be here and true"));

        let mut updated_card = PassthroughCard::update_status(
            created_card.id,
            PassthroughCardStatus::Paused
        ).await.expect("should update");

        assert_eq!(user.id, updated_card.user_id);
        assert_eq!(expected_date, updated_card.expiration);
        assert_eq!(PassthroughCardStatus::Paused, updated_card.passthrough_card_status);
        assert_eq!(PassthroughCardType::Virtual, updated_card.passthrough_card_type);
        assert_eq!(LAST_FOUR, updated_card.last_four);
        assert!(updated_card.is_active.expect("should be here and true"));

        updated_card = PassthroughCard::update_status(
            created_card.id,
            PassthroughCardStatus::Closed
        ).await.expect("should update");

        assert_eq!(user.id, updated_card.user_id);
        assert_eq!(expected_date, updated_card.expiration);
        assert_eq!(PassthroughCardStatus::Closed, updated_card.passthrough_card_status);
        assert_eq!(PassthroughCardType::Virtual, updated_card.passthrough_card_type);
        assert_eq!(LAST_FOUR, updated_card.last_four);
        assert!(updated_card.is_active.is_none());
    }


    #[test]
    async fn test_create_insertable_card_fails_for_same_token() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let expected_date = NaiveDate::from_ymd_opt(2026, 9, 1).expect("date should create");
        let token = Uuid::new_v4();
        let card = create_mock_lithic_card_with_params(
            &token,
            &EXP_MONTH,
            &EXP_YEAR,
            &LAST_FOUR
        );
        let insertable_card = InsertablePassthroughCard::try_from((card, &user)).expect("converts");
        let created_card = PassthroughCard::create(insertable_card.clone()).await.expect("inserts");
        assert_eq!(user.id, created_card.user_id);
        assert_eq!(expected_date, created_card.expiration);
        assert_eq!(PassthroughCardStatus::Open, created_card.passthrough_card_status);
        assert_eq!(PassthroughCardType::Virtual, created_card.passthrough_card_type);
        assert_eq!(LAST_FOUR, created_card.last_four);
        assert!(created_card.is_active.expect("should be here and true"));

        let created_error = PassthroughCard::create(
            insertable_card.clone(),
        ).await.expect_err("error expected to be thrown here");

        assert_eq!(DataError::Conflict("test".into()), created_error);
    }

    #[test]
    async fn test_create_insertable_card_fails_for_active_already() {
        crate::test_helper::general::init();
        let user = create_user().await;

        let expected_date = NaiveDate::from_ymd_opt(2026, 9, 1).expect("date should create");
        let token = Uuid::new_v4();
        let card = create_mock_lithic_card_with_params(
            &token,
            &EXP_MONTH,
            &EXP_YEAR,
            &LAST_FOUR
        );
        let insertable_card = InsertablePassthroughCard::try_from((card, &user)).expect("converts");
        let created_card = PassthroughCard::create(insertable_card.clone()).await.expect("creates");

        assert_eq!(user.id, created_card.user_id);
        assert_eq!(expected_date, created_card.expiration);
        assert_eq!(PassthroughCardStatus::Open, created_card.passthrough_card_status);
        assert_eq!(PassthroughCardType::Virtual, created_card.passthrough_card_type);
        assert_eq!(LAST_FOUR, created_card.last_four);
        assert!(created_card.is_active.expect("should be here and true"));


        let token2 = Uuid::new_v4();
        let card2 = create_mock_lithic_card_with_params(
            &token2,
            &EXP_MONTH,
            &EXP_YEAR,
            &LAST_FOUR
        );
        let insertable_card2 = InsertablePassthroughCard::try_from((card2, &user)).expect("converts");
        let created_error = PassthroughCard::create(insertable_card.clone()).await.expect_err("conflicts");

        assert_eq!(DataError::Conflict("test".into()), created_error);
    }

    #[test]
    async fn test_list_card_for_user() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let user2 = create_user().await;
        let expected_date = NaiveDate::from_ymd_opt(2026, 9, 1).expect("date should create");


        let token = Uuid::new_v4();
        let card = create_mock_lithic_card_with_params(
            &token,
            &EXP_MONTH,
            &EXP_YEAR,
            &LAST_FOUR
        );
        let insertable_card = InsertablePassthroughCard::try_from((card, &user)).expect("converts");
        let created_card1 = PassthroughCard::create(insertable_card).await.expect("creates");

        let updated_card = PassthroughCard::update_status(created_card1.id, PassthroughCardStatus::Closed).await.expect("updates");
        assert!(updated_card.is_active.is_none());
        assert_eq!(updated_card.passthrough_card_status, PassthroughCardStatus::Closed);

        let token2 = Uuid::new_v4();
        let card2 = create_mock_lithic_card_with_params(
            &token2,
            &EXP_MONTH,
            &EXP_YEAR,
            &LAST_FOUR
        );
        let insertable_card2 = InsertablePassthroughCard::try_from((card2, &user)).expect("converts");
        let created_card2 = PassthroughCard::create(insertable_card2).await.expect("creates");

        let token3 = Uuid::new_v4();
        let card3 = create_mock_lithic_card_with_params(
            &token3,
            &EXP_MONTH,
            &EXP_YEAR,
            &LAST_FOUR
        );
        let insertable_card3 = InsertablePassthroughCard::try_from((card3, &user2)).expect("converts");
        let created_card3 = PassthroughCard::create(insertable_card3).await.expect("creates");

        let cards = PassthroughCard::find_cards_for_user(user.id).await.expect("no error");
        assert_eq!(
            HashSet::from([created_card1.id, created_card2.id]),
            HashSet::from_iter(
                cards.iter()
                    .map(|card| card.id)
                    .collect::<Vec<i32>>()
            )
        );

    }
}