#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::user::{
        entity::{User, UserMessage},
    };
    use footprint::models::CreateUserVaultResponse;
    use crate::footprint::error::FootprintError;
    use crate::footprint::service::{FootprintService, MockFootprintServiceTrait};
    use crate::user::dao::{UserDao, UserDaoTrait};
    use crate::user::error::UserError;
    use crate::user::service::{UserService, UserServiceTrait};
    use actix_web::test;
    use uuid::Uuid;
    use crate::test_helper::user::create_user;

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


    #[test]
    async fn test_get_or_create_throws_from_db() {
        crate::test_helper::general::init();
        let generated_id = Uuid::new_v4();
        // db size limit is 256
        let too_long_fp_id = format!(
            "{}{}{}{}{}{}{}{}{}",
            generated_id, generated_id, generated_id, generated_id, generated_id,
            generated_id, generated_id, generated_id, generated_id
        );
        let footprint_response = CreateUserVaultResponse {
            id: too_long_fp_id.to_string()
        };
        let mut fp_mock = MockFootprintServiceTrait::new();
        fp_mock.expect_add_vault_for_user()
            .times(1)
            .return_once(move || Ok(footprint_response));

        let mut service = Arc::new(UserService::new_with_services(
            Arc::new(fp_mock)
        ));

        let returned_error = service.clone().get_or_create(
            AUTH0_ID,
            EMAIL
        ).await.expect_err("no error");
        assert_eq!(UserError::Unexpected("test".into()), returned_error);
    }

    #[test]
    #[ignore]
    async fn test_get_or_create_throws_on_data_error_is_not_not_found() {
        crate::test_helper::general::init();
        let footprint_response = CreateUserVaultResponse {
            id: FOOTPRINT_ID.to_string()
        };
        let mut fp_mock = MockFootprintServiceTrait::new();
        /*
        let mut user_dao = MockUserDaoTrait::new();
        user_dao.expect_find_by_auth0_id()
            .times(1)
            .return_once(move |_| Err(DataError::Unexpected("Unexpected".into())));
         */

        let mut service = Arc::new(UserService::new_with_services(
            Arc::new(fp_mock),
        ));

        let returned_error = service.clone().get_or_create(
            AUTH0_ID,
            EMAIL
        ).await.expect_err("no error");
        assert_eq!(UserError::Unexpected("test".into()), returned_error);
    }

    #[test]
    async fn test_find_by_internal_id_finds() {
        crate::test_helper::general::init();
        let created = create_user().await;
        let fp_mock = MockFootprintServiceTrait::new();
        let service = Arc::new(UserService::new_with_services(
            Arc::new(fp_mock)
        ));
        let found = service.clone().find_by_internal_id(created.id).await.expect("Should find");
        assert_eq!(found.public_id, created.public_id);
        assert_eq!(found.id, created.id);
        assert_eq!(found.footprint_vault_id, created.footprint_vault_id);
    }

    #[test]
    async fn test_find_by_internal_id_errors_not_found() {
        crate::test_helper::general::init();
        let fp_mock = MockFootprintServiceTrait::new();
        let service = Arc::new(UserService::new_with_services(
            Arc::new(fp_mock)
        ));
        let error = service.clone().find_by_internal_id(1000028).await.expect_err("Should not find");
        assert_eq!(UserError::NotFound("test".into()), error);
    }

    #[test]
    #[ignore]
    // TODO: how to test this without mocking db
    async fn test_find_by_internal_id_errors_unexpected() {
        crate::test_helper::general::init();
        let fp_mock = MockFootprintServiceTrait::new();
        /*
        let mut user_mock = MockUserDaoTrait::new();
        user_mock.expect_find_by_internal_id()
            .times(1)
            .return_once(move |_| Err(DataError::Unexpected("test".into())));
         */
        let service = Arc::new(UserService::new_with_services(
            Arc::new(fp_mock)
        ));
        let error = service.clone().find_by_internal_id(-1).await.expect_err("Should not find");
        assert_eq!(UserError::Unexpected("test".into()), error);
    }
}