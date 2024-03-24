use crate::user::model::{
    UserModel as User
};
use super::constant::{
    USER_EMAIL,
    USER_AUTH0_ID,
    USER_FOOTPRINT_VAULT_ID
};

pub fn create_mock_user() -> User {
    User {
        id: 1,
        public_id: Default::default(),
        footprint_vault_id: USER_FOOTPRINT_VAULT_ID.to_string()
    }
}