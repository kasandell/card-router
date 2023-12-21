use crate::user::entity::User;
use crate::api_error::ApiError;
use crate::passthrough_card::constant::PassthroughCardStatus;
use crate::passthrough_card::entity::PassthroughCard;

pub struct Engine {}

impl Engine {
    pub fn issue_card_to_user(
        user: User
    ) -> Result<(), ApiError> {
        Ok(())
    }

    pub fn update_card_status(
        user: User,
        status: PassthroughCardStatus
    ) -> Result<(), ApiError> {
        Ok(())
    }
}