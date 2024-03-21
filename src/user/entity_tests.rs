#[cfg(test)]
mod test {
    use uuid::Uuid;
    use crate::error::error_type::ErrorType;
    use crate::user::dao::{UserDao, UserDaoTrait};
    use crate::user::entity::{User, UserMessage};

    pub const EMAIL: &str = "test@email.com";
    pub const EMAIL2: &str = "test2@email.com";
    pub const AUTH0_ID: &str = "AUTH0_ID";
    pub const AUTH0_ID2: &str = "AUTH0_ID2";
    pub const FOOTPRINT_ID: &str = "FOOTPRINT_ID";
    pub const FOOTPRINT_ID2: &str = "FOOTPRINT_ID2";

    #[actix_web::test]
    async fn test_create() {
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

    #[actix_web::test]
    async fn test_create_from_entity() {
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

    #[actix_web::test]
    async fn test_create_dupe_email() {
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
        assert_eq!(dupe_error.error_type, ErrorType::Conflict);
        user.delete_self().await.expect("deletes");
    }

    #[actix_web::test]
    async fn test_create_dupe_footprint() {
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
        assert_eq!(dupe_error.error_type, ErrorType::Conflict);
        user.delete_self().await.expect("deletes");
    }

    #[actix_web::test]
    async fn test_create_dupe_auth0() {
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
        assert_eq!(dupe_error.error_type, ErrorType::Conflict);
        user.delete_self().await.expect("deletes");
    }

    #[actix_web::test]
    async fn test_find_uuid_finds() {
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
        user.delete_self().await.expect("deletes");
    }

    #[actix_web::test]
    async fn test_find_uuid_does_not_find() {
        let dao = UserDao::new();
        let found_err = dao.find(&Uuid::new_v4()).await.expect_err("Should not find");
        assert_eq!(found_err.error_type, ErrorType::NotFound);
    }

    #[actix_web::test]
    async fn test_find_email_finds() {
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
        user.delete_self().await.expect("deletes");
    }

    #[actix_web::test]
    async fn test_find_email_does_not_find() {
        let dao = UserDao::new();
        let found_err = dao.find_by_email(EMAIL).await.expect_err("Should not find");
        assert_eq!(found_err.error_type, ErrorType::NotFound);
    }

    #[actix_web::test]
    async fn test_find_auth0_finds() {
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
        user.delete_self().await.expect("deletes");
    }

    #[actix_web::test]
    async fn test_find_auth0_does_not_find() {
        let dao = UserDao::new();
        let found_err = dao.find_by_auth0_id(AUTH0_ID).await.expect_err("Should not find");
        assert_eq!(found_err.error_type, ErrorType::NotFound);
    }

    #[actix_web::test]
    async fn test_find_internal_finds() {
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
        user.delete_self().await.expect("deletes");
    }

    #[actix_web::test]
    async fn test_find_internal_does_not_find() {
        let dao = UserDao::new();
        let found_err = dao.find_by_internal_id(1).await.expect_err("Should not find");
        assert_eq!(found_err.error_type, ErrorType::NotFound);
    }

    #[actix_web::test]
    async fn test_update_works() {}

    #[actix_web::test]
    async fn test_update_fails() {}

}