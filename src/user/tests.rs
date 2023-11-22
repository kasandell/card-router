#[cfg(test)]
mod tests {
    use crate::user::{
        entity::{User, UserMessage},
        config::config
    };
    use actix_web::{test::{self, TestRequest}, App, web, body::to_bytes, web::Bytes};
    use serde_json::{json, Value, Result};

    trait BodyTest {
        fn as_str(&self) -> &str;
        fn as_json(&self) -> Value;
    }

    impl BodyTest for Bytes {
        fn as_str(&self) -> &str {
            std::str::from_utf8(self).unwrap()
        }

        fn as_json(&self) -> Value {
            serde_json::from_str(self.as_str()).unwrap()
        }
    }

    #[actix_web::test]
    async fn test_user() {
        crate::test::init();

        let request_body = json!({
            "email": "tore@cloudmaker.dev",
            "password": "test",
        });

        let mut app = test::init_service(App::new().configure(config)).await;

        let resp = TestRequest::post().uri("/").set_json(&request_body).send_request(&mut app).await;
        assert!(resp.status().is_success(), "Failed to create user");
        let user: User = test::read_body_json(resp).await;
        assert_eq!(user.email, "tore@cloudmaker.dev", "Found wrong user");
        assert!(!user.public_id.is_nil());

        let resp = TestRequest::post().uri("/").set_json(&request_body).send_request(&mut app).await;
        assert!(resp.status().is_client_error(), "Should not be possible to create user with same email twice");
    }

    #[actix_web::test]
    async fn test_list() {
        crate::test::init(); 
        User::create(
            UserMessage {
                email: "test@example.com".to_owned(),
                password: "1234".to_owned()
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