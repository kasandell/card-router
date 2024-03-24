use std::sync::Arc;
use footprint::models::CreateUserVaultResponse;
use uuid::Uuid;
use crate::footprint::service::MockFootprintServiceTrait;
use crate::user::model::{
    UserModel as User
};
use crate::user::service::{UserService, UserServiceTrait};
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

fn generate_fake_email() -> String {
    return format!("{}@test.com", Uuid::new_v4().to_string().replace("-", ""));
}

pub async fn create_user() -> User {
    let mut fp_mock = MockFootprintServiceTrait::new();
    fp_mock.expect_add_vault_for_user()
        .times(1)
        .return_once(move || Ok(CreateUserVaultResponse {
            id: Uuid::new_v4().to_string(),
        }));
    let user_service = Arc::new(UserService::new_with_services(
        Arc::new(fp_mock)
    ));
    return user_service.clone().get_or_create(
        &Uuid::new_v4().to_string(),
        &generate_fake_email()
    ).await.expect("Should create user");
}