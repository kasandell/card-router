#[cfg(test)]
mod test {
    use uuid::Uuid;
    use crate::error::data_error::DataError;
    use crate::user::dao::{UserDao, UserDaoTrait};
    use crate::user::entity::{User, UserMessage};
    use actix_web::test;
    use crate::test_helper::user::{create_mock_user, create_user};


    pub const EMAIL: &str = "test@email.com";
    pub const EMAIL2: &str = "test2@email.com";
    pub const AUTH0_ID: &str = "AUTH0_ID";
    pub const AUTH0_ID2: &str = "AUTH0_ID2";
    pub const FOOTPRINT_ID: &str = "FOOTPRINT_ID";
    pub const FOOTPRINT_ID2: &str = "FOOTPRINT_ID2";

    #[test]
    async fn test_create() {
        crate::test_helper::general::init();
        let dao = UserDao::new();
        let user = dao.create(
            &UserMessage {
                email: EMAIL,
                auth0_user_id: AUTH0_ID,
                footprint_vault_id: FOOTPRINT_ID,
            }
        ).await.expect("create user");
        assert_eq!(user.email.as_str(), EMAIL);
        assert_eq!(user.auth0_user_id.as_str(), AUTH0_ID);
        assert_eq!(user.footprint_vault_id.as_str(), FOOTPRINT_ID);
        user.delete_self().await.expect("deletes");
    }

    #[test]
    async fn test_create_from_entity() {
        crate::test_helper::general::init();
        let user = User::create(
            &UserMessage {
                email: EMAIL,
                auth0_user_id: AUTH0_ID,
                footprint_vault_id: FOOTPRINT_ID,
            }
        ).await.expect("create user");
        assert_eq!(user.email.as_str(), EMAIL);
        assert_eq!(user.auth0_user_id.as_str(), AUTH0_ID);
        assert_eq!(user.footprint_vault_id.as_str(), FOOTPRINT_ID);
        user.delete_self().await.expect("deletes");
    }

    #[test]
    async fn test_create_dupe_email() {
        crate::test_helper::general::init();
        let dao = UserDao::new();
        let user = dao.create(
            &UserMessage {
                email: EMAIL,
                auth0_user_id: AUTH0_ID,
                footprint_vault_id: FOOTPRINT_ID,
            }
        ).await.expect("create user");
        let dupe_error = dao.create(
            &UserMessage {
                email: EMAIL,
                auth0_user_id: AUTH0_ID2,
                footprint_vault_id: FOOTPRINT_ID2
            }
        ).await.expect_err("should create error");
        assert_eq!(DataError::Conflict("Test".into()), dupe_error);
        user.delete_self().await.expect("deletes");
    }

    #[test]
    async fn test_create_dupe_footprint() {
        crate::test_helper::general::init();
        let dao = UserDao::new();
        let user = dao.create(
            &UserMessage {
                email: EMAIL,
                auth0_user_id: AUTH0_ID,
                footprint_vault_id: FOOTPRINT_ID,
            }
        ).await.expect("create user");
        let dupe_error = dao.create(
            &UserMessage {
                email: EMAIL2,
                auth0_user_id: AUTH0_ID2,
                footprint_vault_id: FOOTPRINT_ID
            }
        ).await.expect_err("should create error");
        assert_eq!(DataError::Conflict("Test".into()), dupe_error);
        user.delete_self().await.expect("deletes");
    }

    #[test]
    async fn test_create_dupe_auth0() {
        crate::test_helper::general::init();
        let dao = UserDao::new();
        let user = dao.create(
            &UserMessage {
                email: EMAIL,
                auth0_user_id: AUTH0_ID,
                footprint_vault_id: FOOTPRINT_ID,
            }
        ).await.expect("create user");
        let dupe_error = dao.create(
            &UserMessage {
                email: EMAIL2,
                auth0_user_id: AUTH0_ID,
                footprint_vault_id: FOOTPRINT_ID2
            }
        ).await.expect_err("should create error");
        assert_eq!(DataError::Conflict("Test".into()), dupe_error);
        user.delete_self().await.expect("deletes");
    }

    #[test]
    async fn test_find_uuid_finds() {
        crate::test_helper::general::init();
        let dao = UserDao::new();
        let user = dao.create(
            &UserMessage {
                email: EMAIL,
                auth0_user_id: AUTH0_ID,
                footprint_vault_id: FOOTPRINT_ID,
            }
        ).await.expect("create user");

        let found = dao.find(&user.public_id).await.expect("Should find");
        assert_eq!(found, user);
    }

    #[test]
    async fn test_find_uuid_does_not_find() {
        crate::test_helper::general::init();
        let dao = UserDao::new();
        let found_err = dao.find(&Uuid::new_v4()).await.expect_err("Should not find");
        assert_eq!(DataError::NotFound("Test".into()), found_err);
    }

    #[test]
    async fn test_find_email_finds() {
        crate::test_helper::general::init();
        let dao = UserDao::new();
        let user = dao.create(
            &UserMessage {
                email: EMAIL,
                auth0_user_id: AUTH0_ID,
                footprint_vault_id: FOOTPRINT_ID,
            }
        ).await.expect("create user");

        let found = dao.find_by_email(EMAIL).await.expect("Should find");
        assert_eq!(found, user);
    }

    #[test]
    async fn test_find_email_does_not_find() {
        crate::test_helper::general::init();
        let dao = UserDao::new();
        let found_err = dao.find_by_email(EMAIL).await.expect_err("Should not find");
        assert_eq!(DataError::NotFound("Test".into()), found_err);
    }

    #[test]
    async fn test_find_auth0_finds() {
        crate::test_helper::general::init();
        let dao = UserDao::new();
        let user = dao.create(
            &UserMessage {
                email: EMAIL,
                auth0_user_id: AUTH0_ID,
                footprint_vault_id: FOOTPRINT_ID,
            }
        ).await.expect("create user");

        let found = dao.find_by_auth0_id(AUTH0_ID).await.expect("Should find");
        assert_eq!(found, user);
    }

    #[test]
    async fn test_find_auth0_does_not_find() {
        crate::test_helper::general::init();
        let dao = UserDao::new();
        let found_err = dao.find_by_auth0_id(AUTH0_ID).await.expect_err("Should not find");
        assert_eq!(DataError::NotFound("test".into()), found_err);
    }

    #[test]
    async fn test_find_internal_finds() {
        crate::test_helper::general::init();
        let dao = UserDao::new();
        let user = dao.create(
            &UserMessage {
                email: EMAIL,
                auth0_user_id: AUTH0_ID,
                footprint_vault_id: FOOTPRINT_ID,
            }
        ).await.expect("create user");

        let found = dao.find_by_internal_id(user.id).await.expect("Should find");
        assert_eq!(found, user);
    }

    #[test]
    #[ignore]
    // test reliant on data setup
    async fn test_find_internal_does_not_find() {
        crate::test_helper::general::init();
        let dao = UserDao::new();
        let found_err = dao.find_by_internal_id(1).await.expect_err("Should not find");
        assert_eq!(DataError::NotFound("test".into()), found_err);
    }

    #[test]
    async fn test_update_works() {
        crate::test_helper::general::init();
        let dao = UserDao::new();
        let user = create_user().await;
        let new_email = "test@test-email.com";
        let new_auth0 = "auth0_12345";
        let new_footprint = "footprint_12345";
        let user = dao.update(&user.public_id, &UserMessage {
            email: new_email,
            auth0_user_id: new_auth0,
            footprint_vault_id: new_footprint
        }).await.expect("ok");

        assert_eq!(user.footprint_vault_id, new_footprint);
        assert_eq!(user.auth0_user_id, new_auth0);
        assert_eq!(user.email, new_email);
    }
    #[test]
    async fn test_update_fails() {
        crate::test_helper::general::init();
        let dao = UserDao::new();
        let user = create_mock_user();
        let new_email = "test@test-email.com";
        let new_auth0 = "auth0_12345";
        let new_footprint = "footprint_12345";
        let error = dao.update(&user.public_id, &UserMessage {
            email: new_email,
            auth0_user_id: new_auth0,
            footprint_vault_id: new_footprint
        }).await.expect_err("error");
        assert_eq!(DataError::NotFound("test".into()), error);
    }
}