#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use chrono::NaiveDate;
    use crate::error_type::ErrorType;
    use crate::passthrough_card::constant::PassthroughCardStatus;
    use crate::test_helper::initialize_user;
    use crate::passthrough_card::entity::{
        LithicCard,
        PassthroughCard,
        PassthroughCardStatusUpdate,
        InsertablePassthroughCard
    };
    use crate::user::entity::{User, UserMessage};

    #[actix_web::test]
    async fn test_create_insertable_card() {
        crate::test::init();
        let user = initialize_user().await;
        let token = "12345";
        let last_four = "1234";
        let exp_month = "09";
        let exp_year = "2026";
        let expected_date = NaiveDate::from_ymd_opt(2026, 9, 1).expect("date should create");
        let card = LithicCard {
            token: token.to_string(),
            last_four: last_four.to_string(),
            exp_month: exp_month.to_string(),
            exp_year: exp_year.to_string()
        };
        let insertable_card = InsertablePassthroughCard::from(card.clone());
        assert_eq!(expected_date, insertable_card.expiration);
        assert_eq!(token, insertable_card.token);
        assert_eq!("OPEN", insertable_card.passthrough_card_status);
        assert_eq!("VIRTUAL", insertable_card.passthrough_card_type);
        let created_card = PassthroughCard::create(
            card,
            &user
        ).await.expect("card should create");

        assert_eq!(user.id, created_card.user_id);
        assert_eq!(expected_date, created_card.expiration);
        assert_eq!("OPEN", created_card.passthrough_card_status);
        assert_eq!("VIRTUAL", created_card.passthrough_card_type);
        assert_eq!(last_four, created_card.last_four);
        assert!(created_card.is_active.expect("should be here and true"));
        created_card.delete_self().await.expect("card should delete");
        user.delete_self().await.expect("User should delete");
    }

    #[actix_web::test]
    async fn test_status_update() {
        crate::test::init();
        let user = initialize_user().await;
        let token = "12345";
        let last_four = "1234";
        let exp_month = "09";
        let exp_year = "2026";
        let expected_date = NaiveDate::from_ymd_opt(2026, 9, 1).expect("date should create");
        let card = LithicCard {
            token: token.to_string(),
            last_four: last_four.to_string(),
            exp_month: exp_month.to_string(),
            exp_year: exp_year.to_string()
        };
        let insertable_card = InsertablePassthroughCard::from(card.clone());
        assert_eq!(expected_date, insertable_card.expiration);
        assert_eq!(token, insertable_card.token);
        assert_eq!("OPEN", insertable_card.passthrough_card_status);
        assert_eq!("VIRTUAL", insertable_card.passthrough_card_type);
        let created_card = PassthroughCard::create(
            card,
            &user
        ).await.expect("card should create");

        assert_eq!(user.id, created_card.user_id);
        assert_eq!(expected_date, created_card.expiration);
        assert_eq!("OPEN", created_card.passthrough_card_status);
        assert_eq!("VIRTUAL", created_card.passthrough_card_type);
        assert_eq!(last_four, created_card.last_four);
        assert!(created_card.is_active.expect("should be here and true"));

        let mut updated_card = PassthroughCard::update_status(
            created_card.id,
            PassthroughCardStatus::PAUSED
        ).await.expect("should update");

        assert_eq!(user.id, updated_card.user_id);
        assert_eq!(expected_date, updated_card.expiration);
        assert_eq!("PAUSED", updated_card.passthrough_card_status);
        assert_eq!("VIRTUAL", updated_card.passthrough_card_type);
        assert_eq!(last_four, updated_card.last_four);
        assert!(updated_card.is_active.expect("should be here and true"));

        updated_card = PassthroughCard::update_status(
            created_card.id,
            PassthroughCardStatus::CLOSED
        ).await.expect("should update");

        assert_eq!(user.id, updated_card.user_id);
        assert_eq!(expected_date, updated_card.expiration);
        assert_eq!("CLOSED", updated_card.passthrough_card_status);
        assert_eq!("VIRTUAL", updated_card.passthrough_card_type);
        assert_eq!(last_four, updated_card.last_four);
        assert!(updated_card.is_active.is_none());

        updated_card.delete_self().await.expect("card should delete");
        user.delete_self().await.expect("User should delete");
    }


    #[actix_web::test]
    async fn test_create_insertable_card_fails_for_same_token() {
        crate::test::init();
        let user = initialize_user().await;
        let token = "12345";
        let last_four = "1234";
        let exp_month = "09";
        let exp_year = "2026";
        let expected_date = NaiveDate::from_ymd_opt(2026, 9, 1).expect("date should create");
        let card = LithicCard {
            token: token.to_string(),
            last_four: last_four.to_string(),
            exp_month: exp_month.to_string(),
            exp_year: exp_year.to_string()
        };
        let created_card = PassthroughCard::create(
            card.clone(),
            &user
        ).await.expect("card should create");
        assert_eq!(user.id, created_card.user_id);
        assert_eq!(expected_date, created_card.expiration);
        assert_eq!("OPEN", created_card.passthrough_card_status);
        assert_eq!("VIRTUAL", created_card.passthrough_card_type);
        assert_eq!(last_four, created_card.last_four);
        assert!(created_card.is_active.expect("should be here and true"));

        let created_error = PassthroughCard::create(
            card.clone(),
            &user
        ).await.expect_err("error expected to be thrown here");

        assert_eq!(ErrorType::Conflict, created_error.error_type);

        created_card.delete_self().await.expect("card should delete");
        user.delete_self().await.expect("User should delete");
    }

    #[actix_web::test]
    async fn test_create_insertable_card_fails_for_active_already() {
        crate::test::init();
        let user = initialize_user().await;
        let token = "12345";
        let token2 = "23456";
        let last_four = "1234";
        let exp_month = "09";
        let exp_year = "2026";
        let expected_date = NaiveDate::from_ymd_opt(2026, 9, 1).expect("date should create");
        let card = LithicCard {
            token: token.to_string(),
            last_four: last_four.to_string(),
            exp_month: exp_month.to_string(),
            exp_year: exp_year.to_string()
        };
        let created_card = PassthroughCard::create(
            card.clone(),
            &user
        ).await.expect("card should create");
        assert_eq!(user.id, created_card.user_id);
        assert_eq!(expected_date, created_card.expiration);
        assert_eq!("OPEN", created_card.passthrough_card_status);
        assert_eq!("VIRTUAL", created_card.passthrough_card_type);
        assert_eq!(last_four, created_card.last_four);
        assert!(created_card.is_active.expect("should be here and true"));

        let card2 = LithicCard {
            token: token2.to_string(),
            last_four: last_four.to_string(),
            exp_month: exp_month.to_string(),
            exp_year: exp_year.to_string()
        };

        let created_error = PassthroughCard::create(
            card.clone(),
            &user
        ).await.expect_err("error expected to be thrown here");

        assert_eq!(ErrorType::Conflict, created_error.error_type);

        created_card.delete_self().await.expect("card should delete");
        user.delete_self().await.expect("User should delete");
    }

    #[actix_web::test]
    async fn test_list_card_for_user() {
        crate::test::init();
        let user = initialize_user().await;
        let user2 = User::create(
            &UserMessage {
                email: "kyle2@gmail.com",
                auth0_user_id: "1234"
            }
        ).await.expect("User should be created");
        let token = "12345";
        let token2 = "23456";
        let token3 = "34567";
        let last_four = "1234";
        let exp_month = "09";
        let exp_year = "2026";
        let expected_date = NaiveDate::from_ymd_opt(2026, 9, 1).expect("date should create");
        let card1 = LithicCard {
            token: token.to_string(),
            last_four: last_four.to_string(),
            exp_month: exp_month.to_string(),
            exp_year: exp_year.to_string()
        };

        let mut created_card1 = PassthroughCard::create(
            card1.clone(),
            &user
        ).await.expect("card should create");
        created_card1 = PassthroughCard::update_status(
            created_card1.id,
            PassthroughCardStatus::CLOSED
        ).await.expect("should be fine");

        let card2 = LithicCard {
            token: token2.to_string(),
            last_four: last_four.to_string(),
            exp_month: exp_month.to_string(),
            exp_year: exp_year.to_string()
        };

        let created_card2 = PassthroughCard::create(
            card2.clone(),
            &user
        ).await.expect("should create");

        let card3 = LithicCard {
            token: token3.to_string(),
            last_four: last_four.to_string(),
            exp_month: exp_month.to_string(),
            exp_year: exp_year.to_string()
        };

        let created_card3 = PassthroughCard::create(
            card3.clone(),
            &user2
        ).await.expect("should create");

        let cards = PassthroughCard::find_cards_for_user(user.id).await.expect("no error");
        assert_eq!(
            HashSet::from([created_card1.id, created_card2.id]),
            HashSet::from_iter(
                cards.iter()
                    .map(|card| card.id)
                    .collect::<Vec<i32>>()
            )
        );


        created_card1.delete_self().await.expect("card should delete");
        created_card2.delete_self().await.expect("card should delete");
        created_card3.delete_self().await.expect("card should delete");
        user2.delete_self().await.expect("user should delete");
        user.delete_self().await.expect("User should delete");
    }

}