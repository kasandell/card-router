#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use chrono::NaiveDate;
    use crate::passthrough_card::constant::{PassthroughCardStatus, PassthroughCardType};
    use crate::test_helper::user::create_user;
    use crate::passthrough_card::entity::{
        LithicCard,
        PassthroughCard,
        InsertablePassthroughCard
    };
    use crate::user::entity::{User, UserMessage};

    const TOKEN: &str = "12345";
    const TOKEN2: &str = "12346";
    const TOKEN3: &str = "12347";
    const LAST_FOUR: &str = "1234";
    const EXP_MONTH: &str = "09";
    const EXP_YEAR: &str = "2026";

    #[actix_web::test]
    async fn test_create_insertable_card() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let expected_date = NaiveDate::from_ymd_opt(2026, 9, 1).expect("date should create");
        let card = LithicCard {
            token: TOKEN.to_string(),
            last_four: LAST_FOUR.to_string(),
            exp_month: EXP_MONTH.to_string(),
            exp_year: EXP_YEAR.to_string()
        };
        let insertable_card = InsertablePassthroughCard::convert_from_lithic_card(&card.clone()).expect("should convert");
        assert_eq!(expected_date, insertable_card.expiration);
        assert_eq!(TOKEN, insertable_card.token);
        assert_eq!(PassthroughCardStatus::Open, insertable_card.passthrough_card_status);
        assert_eq!(PassthroughCardType::Virtual, insertable_card.passthrough_card_type);
        let created_card = PassthroughCard::create(
            card,
            &user
        ).await.expect("card should create");

        assert_eq!(user.id, created_card.user_id);
        assert_eq!(expected_date, created_card.expiration);
        assert_eq!(PassthroughCardStatus::Open, created_card.passthrough_card_status);
        assert_eq!(PassthroughCardType::Virtual, created_card.passthrough_card_type);
        assert_eq!(LAST_FOUR, created_card.last_four);
        assert!(created_card.is_active.expect("should be here and true"));
        created_card.delete_self().await.expect("card should delete");
        user.delete_self().await.expect("User should delete");
    }
    #[actix_web::test]
    async fn test_status_update() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let expected_date = NaiveDate::from_ymd_opt(2026, 9, 1).expect("date should create");
        let card = LithicCard {
            token: TOKEN.to_string(),
            last_four: LAST_FOUR.to_string(),
            exp_month: EXP_MONTH.to_string(),
            exp_year: EXP_YEAR.to_string()
        };
        let insertable_card = InsertablePassthroughCard::convert_from_lithic_card(&card.clone()).expect("should convert");
        assert_eq!(expected_date, insertable_card.expiration);
        assert_eq!(TOKEN, insertable_card.token);
        assert_eq!(PassthroughCardStatus::Open, insertable_card.passthrough_card_status);
        assert_eq!(PassthroughCardType::Virtual, insertable_card.passthrough_card_type);
        let created_card = PassthroughCard::create(
            card,
            &user
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

        updated_card.delete_self().await.expect("card should delete");
        user.delete_self().await.expect("User should delete");
    }


    #[actix_web::test]
    async fn test_create_insertable_card_fails_for_same_token() {
        crate::test_helper::general::init();
        let user = create_user().await;
        let expected_date = NaiveDate::from_ymd_opt(2026, 9, 1).expect("date should create");
        let card = LithicCard {
            token: TOKEN.to_string(),
            last_four: LAST_FOUR.to_string(),
            exp_month: EXP_MONTH.to_string(),
            exp_year: EXP_YEAR.to_string()
        };
        let created_card = PassthroughCard::create(
            card.clone(),
            &user
        ).await.expect("card should create");
        assert_eq!(user.id, created_card.user_id);
        assert_eq!(expected_date, created_card.expiration);
        assert_eq!(PassthroughCardStatus::Open, created_card.passthrough_card_status);
        assert_eq!(PassthroughCardType::Virtual, created_card.passthrough_card_type);
        assert_eq!(LAST_FOUR, created_card.last_four);
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
        crate::test_helper::general::init();
        let user = create_user().await;

        let expected_date = NaiveDate::from_ymd_opt(2026, 9, 1).expect("date should create");
        let card = LithicCard {
            token: TOKEN.to_string(),
            last_four: LAST_FOUR.to_string(),
            exp_month: EXP_MONTH.to_string(),
            exp_year: EXP_YEAR.to_string()
        };
        let created_card = PassthroughCard::create(
            card.clone(),
            &user
        ).await.expect("card should create");
        assert_eq!(user.id, created_card.user_id);
        assert_eq!(expected_date, created_card.expiration);
        assert_eq!(PassthroughCardStatus::Open, created_card.passthrough_card_status);
        assert_eq!(PassthroughCardType::Virtual, created_card.passthrough_card_type);
        assert_eq!(LAST_FOUR, created_card.last_four);
        assert!(created_card.is_active.expect("should be here and true"));

        let card2 = LithicCard {
            token: TOKEN2.to_string(),
            last_four: LAST_FOUR.to_string(),
            exp_month: EXP_MONTH.to_string(),
            exp_year: EXP_YEAR.to_string()
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
        crate::test_helper::general::init();
        let user = create_user().await;
        let user2 = User::create(
            &UserMessage {
                email: "kyle2@gmail.com",
                auth0_user_id: "12345",
                footprint_vault_id: "test_vault_2"
            }
        ).await.expect("User should be created");
        let expected_date = NaiveDate::from_ymd_opt(2026, 9, 1).expect("date should create");
        let card1 = LithicCard {
            token: TOKEN.to_string(),
            last_four: LAST_FOUR.to_string(),
            exp_month: EXP_MONTH.to_string(),
            exp_year: EXP_YEAR.to_string()
        };

        let mut created_card1 = PassthroughCard::create(
            card1.clone(),
            &user
        ).await.expect("card should create");
        created_card1 = PassthroughCard::update_status(
            created_card1.id,
            PassthroughCardStatus::Closed
        ).await.expect("should be fine");

        let card2 = LithicCard {
            token: TOKEN2.to_string(),
            last_four: LAST_FOUR.to_string(),
            exp_month: EXP_MONTH.to_string(),
            exp_year: EXP_YEAR.to_string()
        };

        let created_card2 = PassthroughCard::create(
            card2.clone(),
            &user
        ).await.expect("should create");

        let card3 = LithicCard {
            token: TOKEN3.to_string(),
            last_four: LAST_FOUR.to_string(),
            exp_month: EXP_MONTH.to_string(),
            exp_year: EXP_YEAR.to_string()
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