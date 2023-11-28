use crate::user::entity::{User, UserMessage};
#[cfg(test)]
pub fn initialize_user() -> User {
    User::create(
        UserMessage {
            email: "test@example.com".to_string(),
            password: "1234".to_string()
        }
    ).expect("User should be created")
}