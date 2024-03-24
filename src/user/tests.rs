#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::user::{
        entity::{User, UserMessage},
    };
    use footprint::models::CreateUserVaultResponse;
    use crate::footprint::error::FootprintError;
    use crate::footprint::service::{FootprintService, MockFootprintServiceTrait};
    use crate::user::dao::{MockUserDaoTrait, UserDao, UserDaoTrait};
    use crate::user::error::UserError;
    use crate::user::service::{UserService, UserServiceTrait};
    use actix_web::test;

    const FOOTPRINT_ID: &str = "footprint_123";
    const AUTH0_ID: &str = "auth0_123";
    const EMAIL: &str = "test@email.com";


    #[test]
    async fn test_get_or_create_gets() {
        crate::test_helper::general::init();
        let user = UserDao::new().create(&UserMessage {
            email: EMAIL,
            auth0_user_id: AUTH0_ID,
            footprint_vault_id: FOOTPRINT_ID
        }).await.expect("OK");
        let mut fp_mock = MockFootprintServiceTrait::new();
        fp_mock.expect_add_vault_for_user()
            .times(0);

        let mut service = Arc::new(UserService::new_with_services(
            Arc::new(fp_mock)
        ));

        let returned = service.get_or_create(AUTH0_ID, EMAIL).await.expect("should give user");
        assert_eq!(returned.id, user.id);
        assert_eq!(returned.public_id, user.public_id);
        assert_eq!(returned.footprint_vault_id, user.footprint_vault_id);
    }

    #[test]
    async fn test_get_or_create_creates() {
        crate::test_helper::general::init();
        let footprint_response = CreateUserVaultResponse {
            id: FOOTPRINT_ID.to_string()
        };
        let mut fp_mock = MockFootprintServiceTrait::new();
        fp_mock.expect_add_vault_for_user()
            .times(1)
            .return_once(move || Ok(footprint_response));

        let mut service = Arc::new(UserService::new_with_services(
            Arc::new(fp_mock)
        ));

        let returned_user = service.clone().get_or_create(
            AUTH0_ID,
            EMAIL
        ).await.expect("no error");
        assert_eq!(returned_user.footprint_vault_id, FOOTPRINT_ID.to_string());
    }

    #[test]
    async fn test_get_or_create_throws() {
        crate::test_helper::general::init();
        let footprint_response = CreateUserVaultResponse {
            id: FOOTPRINT_ID.to_string()
        };
        let mut fp_mock = MockFootprintServiceTrait::new();
        fp_mock.expect_add_vault_for_user()
            .times(1)
            .return_once(move || Err(FootprintError::Unexpected("Test Error".into())));

        let mut service = Arc::new(UserService::new_with_services(
            Arc::new(fp_mock)
        ));

        let returned_error = service.clone().get_or_create(
            AUTH0_ID,
            EMAIL
        ).await.expect_err("no error");
        assert_eq!(UserError::Unexpected("test".into()), returned_error);
    }
}