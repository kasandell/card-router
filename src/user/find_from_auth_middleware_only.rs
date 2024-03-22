use crate::user::error::UserError;
use crate::user::model::UserModel;
use crate::user::entity::User;

pub async fn find_by_auth0_id(auth0_id: &str) -> Result<UserModel, UserError> {
    Ok(
        User::find_by_auth0_id(auth0_id)
            .await.map_err(|e| UserError::Unexpected(e.into()))?.into()
    )
}