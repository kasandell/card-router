#[cfg(test)]
mod tests {
    use crate::user::{
        entity::{User, UserMessage},
        config::config
    };
    use crate::test::BodyTest;
    use actix_web::{test::{self, TestRequest}, App, body::to_bytes};
    use serde_json::json;

    // TODO: have to init all services at controller level
    //#[actix_web::test]
    async fn test_dupe_create() {
        crate::test::init();
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

    //#[actix_web::test]
    async fn test_list() {
        crate::test::init(); 
        let user = User::create(
            &UserMessage {
                email: "test@example.com",
                auth0_user_id: "1234",
            }
        ).await.expect("User should exist");
        let public_id = user.public_id;
        let mut app = test::init_service(App::new().configure(config)).await;
        let resp = TestRequest::get().uri("/list/").send_request(&mut app).await;
        let body = to_bytes(resp.into_body()).await.unwrap();
        let body_json = body.as_json();
        assert!(body_json.is_array());
        // assert_eq!(body_json.as_array().unwrap().len(), 1);
        user.delete_self().await.expect("Should delete");
        assert!(User::find(&public_id).await.is_err())
    }
}