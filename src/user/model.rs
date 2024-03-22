use chrono::NaiveDateTime;
use serde::Deserialize;
use serde_with::serde_derive::Serialize;
use uuid::Uuid;
use crate::user::entity::User;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserModel {
    pub id: i32,
    pub public_id: Uuid,
    pub footprint_vault_id: String,
}


impl From<User> for UserModel {
    fn from(user: User) -> Self {
        UserModel {
            id: user.id,
            public_id: user.public_id,
            footprint_vault_id: user.footprint_vault_id
        }
    }
}