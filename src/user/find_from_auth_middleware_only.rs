use crate::user::error::UserError;
use crate::user::model::UserModel;
use crate::user::entity::User;

#[cfg_attr(feature="trace-detail", tracing::instrument(skip_all))]
pub async fn find_by_auth0_id(auth0_id: &str) -> Result<UserModel, UserError> {
    Ok(
        User::find_by_auth0_id(auth0_id)
            .await.map_err(|e| UserError::Unexpected(e.into()))?.into()
    )
}


#[cfg(test)]
mod test {
    use actix_web::test;
    use futures_util::future::err;
    use crate::test_helper::user::create_user;
    use crate::user::entity::User;
    use crate::user::error::UserError;
    use crate::user::find_from_auth_middleware_only::find_by_auth0_id;

    #[test]
    async fn test_find_by_auth0_works() {
        crate::test_helper::general::init();
        let user_model = create_user().await;
        let user_db = User::find_by_internal_id(user_model.id).await.expect("should exist");
        let found = find_by_auth0_id(&user_db.auth0_user_id).await.expect("should find");
        assert_eq!(found.id, user_model.id);
        assert_eq!(found.footprint_vault_id, user_model.footprint_vault_id);
        assert_eq!(found.id, user_db.id);
        assert_eq!(found.footprint_vault_id, user_db.footprint_vault_id);
    }

    #[test]
    async fn test_find_by_auth0_fails() {
        crate::test_helper::general::init();
        let error = find_by_auth0_id("1234").await.expect_err("should not find");
        assert_eq!(UserError::Unexpected("test".into()), error);
    }
}