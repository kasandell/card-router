use serde::{Deserialize, Serialize};
use crate::user::model::UserModel as User;

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

#[cfg(test)]
mod test {
    use crate::user::response::UserResponse;

    #[test]
    pub fn test_from_user() {
        let model = crate::test_helper::user::create_mock_user();
        let resp = UserResponse::from(&model);
        assert_eq!(resp.public_id, model.public_id.to_string());
    }
}