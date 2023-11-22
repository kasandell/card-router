#[cfg(test)]
mod tests {
    use crate::user::{
        entity::{User, UserMessage},
        config::config
    };
    use crate::test::BodyTest;
    use actix_web::{test::{self, TestRequest}, App, web, body::to_bytes, web::Bytes};
    use serde_json::{json, Value, Result};

    #[actix_web::test]
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
        assert!(resp.status().is_client_error(), "Should not be possible to create user with same email twice");
    }

    #[actix_web::test]
    async fn test_list() {
        crate::test::init(); 
        let _ = User::create(
            UserMessage {
                email: "test@example.com".to_owned(),
                password: "password".to_owned()
            }
        );

        let mut app = test::init_service(App::new().configure(config)).await;

        let resp = TestRequest::get().uri("/list/").send_request(&mut app).await;

        let body = to_bytes(resp.into_body()).await.unwrap();
        let body_json = body.as_json();
        assert!(body_json.is_array());
        assert_eq!(body_json.as_array().unwrap().len(), 1);
    }
}