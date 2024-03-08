use serde::{Deserialize, Serialize};
use crate::user::entity::User;

#[derive(Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub public_id: String
}

impl From<&User> for UserResponse {
    fn from(user: &User) -> Self {
        UserResponse {
            public_id: user.public_id.to_string()
        }
    }
}