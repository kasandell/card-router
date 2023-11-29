#[cfg(test)]
mod tests {
    use crate::test_helper::initialize_user;
    use actix_web::{test::{self, TestRequest}, App, body::to_bytes};
    use crate::rule_engine::entity::Rule;
    use serde_json::json;
    use crate::user::entity::User;

    #[actix_web::test]
    async fn test_fulter_rules() {
        crate::test::init();
        let user = initialize_user();
        User::delete(user.public_id);
    }
}