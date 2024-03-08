use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
}