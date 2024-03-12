#[cfg(test)]
mod tests {
    use crate::user::{
        entity::{User, UserMessage},
        config::config
    };
    use crate::test_helper::general::BodyTest;
    use actix_web::{test::{self, TestRequest}, App, body::to_bytes};
    use serde_json::json;

    // TODO: have to init all services at controller level
    //#[actix_web::test]
    async fn test_dupe_create() {
        crate::test_helper::general::init();
        let request_body = json!({
            "email": "test@example.com",
            "password": "test",
        });

        let mut app = test::init_service(App::new().configure(config)).await;
        let resp = TestRequest::post().uri("/").set_json(&request_body).send_request(&mut app).await;
        assert!(resp.status().is_success(), "Failed to create user");
        let user: User = test::read_body_json(resp).await;
        assert_eq!(user.email, "test@example.com", "Found wrong user");
        assert!(!user.public_id.is_nil());

        let resp = TestRequest::post().uri("/").set_json(&request_body).send_request(&mut app).await;
        // TODO: data exceptions are bubbling up as 500, but for conflict we want 409
        assert!(resp.status().is_client_error(), "Should not be possible to create user with same email twice");
        user.delete_self().await.expect("user should delete");
        assert!(User::find(&user.public_id).await.is_err())
    }
}