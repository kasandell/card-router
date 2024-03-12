use crate::user::entity::{User, UserMessage};
use crate::util::db;
use super::constant::{
    USER_EMAIL,
    USER_AUTH0_ID,
    USER_FOOTPRINT_VAULT_ID
};

pub fn create_mock_user() -> User {
    User {
        id: 1,
        public_id: Default::default(),
        email: USER_EMAIL.to_string(),
        auth0_user_id: USER_AUTH0_ID.to_string(),
        created_at: Default::default(),
        updated_at: Default::default(),
        footprint_vault_id: USER_FOOTPRINT_VAULT_ID.to_string()
    }
}

pub async fn create_user() -> User {
    User::create(
        &UserMessage {
            email: USER_EMAIL,
            auth0_user_id: USER_AUTH0_ID,
            footprint_vault_id: USER_FOOTPRINT_VAULT_ID
        }
    ).await.expect("should create user")
}