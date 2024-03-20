#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::user::{
        entity::{User, UserMessage},
        config::config
    };
    use crate::test_helper::general::BodyTest;
    use actix_web::{test::{self, TestRequest}, App, body::to_bytes};
    use footprint::models::CreateUserVaultResponse;
    use mockall::predicate::eq;
    use serde_json::json;
    use crate::error::data_error::DataError;
    use crate::error::error_type::ErrorType;
    use crate::footprint::service::{FootprintService, MockFootprintServiceTrait};
    use crate::test_helper::user::{create_mock_user, create_user};
    use crate::user::dao::MockUserDaoTrait;
    use crate::user::service::{UserService, UserServiceTrait};

    const FOOTPRINT_ID: &str = "footprint_123";
    const AUTH0_ID: &str = "auth0_123";
    const EMAIL: &str = "test@email.com";

    // TODO: have to init all services at controller level
    //#[actix_web::test]
    async fn test_dupe_create() {
        crate::test_helper::general::init();
        let request_body = json!({
            "email": "test@example.com",
            "password": "test",
        });

        let mut app = test::init_service(App::new().configure(config)).await;
        let resp = TestRequest::post().uri("/").set_json(&request_body).send_request(&mut app).await;
        assert!(resp.status().is_success(), "Failed to create user");
        let user: User = test::read_body_json(resp).await;
        assert_eq!(user.email, "test@example.com", "Found wrong user");
        assert!(!user.public_id.is_nil());

        let resp = TestRequest::post().uri("/").set_json(&request_body).send_request(&mut app).await;
        // TODO: data exceptions are bubbling up as 500, but for conflict we want 409
        assert!(resp.status().is_client_error(), "Should not be possible to create user with same email twice");
        user.delete_self().await.expect("user should delete");
        assert!(User::find(&user.public_id).await.is_err())
    }

    #[actix_web::test]
    async fn test_get_or_create_creates() {
        let footprint_response = CreateUserVaultResponse {
            id: FOOTPRINT_ID.to_string()
        };
        let mut fp_mock = MockFootprintServiceTrait::new();
        fp_mock.expect_add_vault_for_user()
            .times(1)
            .return_const(Ok(footprint_response));

        let mut user = create_mock_user();
        user.footprint_vault_id = FOOTPRINT_ID.to_string();
        user.auth0_user_id = AUTH0_ID.to_string();
        user.email = EMAIL.to_string();
        let mut dao_mock = MockUserDaoTrait::new();
        dao_mock.expect_find_by_auth0_id()
            .times(1)
            .with(eq(AUTH0_ID.clone()))
            .return_const(Err(DataError::new(ErrorType::NotFound, "No user")));

        let expected_msg = UserMessage {
            email: EMAIL.clone(),
            auth0_user_id: AUTH0_ID.clone(),
            footprint_vault_id: FOOTPRINT_ID.clone()
        };

        dao_mock.expect_create()
            .times(1)
            .withf(|user_message| {
                return user_message.email == EMAIL
                    && user_message.auth0_user_id == AUTH0_ID
                    && user_message.footprint_vault_id == FOOTPRINT_ID;
            })
            .return_const(Ok(user.clone()));

        let mut service = Arc::new(UserService::new_with_services(
            Arc::new(dao_mock),
            Arc::new(fp_mock)
        ));

        let returned_user = service.clone().get_or_create(
            AUTH0_ID,
            EMAIL
        ).await.expect("no error");
        assert_eq!(returned_user, user);

    }

    #[actix_web::test]
    async fn test_get_or_create_gets() {
        let footprint_response = CreateUserVaultResponse {
            id: FOOTPRINT_ID.to_string()
        };
        let mut fp_mock = MockFootprintServiceTrait::new();
        fp_mock.expect_add_vault_for_user()
            .times(0);

        let mut user = create_mock_user();
        user.footprint_vault_id = FOOTPRINT_ID.to_string();
        let mut dao_mock = MockUserDaoTrait::new();
        dao_mock.expect_find_by_auth0_id()
            .times(1)
            .with(eq(AUTH0_ID.clone()))
            .return_const(Ok(user.clone()));

        let mut service = Arc::new(UserService::new_with_services(
            Arc::new(dao_mock),
            Arc::new(fp_mock)
        ));

        let returned_user = service.clone().get_or_create(
            AUTH0_ID,
            EMAIL
        ).await.expect("no error");
        assert_eq!(returned_user, user);
    }

    async fn test_get_or_create_throws() {

    }
}