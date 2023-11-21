#[cfg(test)]
mod tests {
   use crate::user::{
    entity::User,
    config::config
   };
   use actix_web::{test::{self, TestRequest}, App, web, body::to_bytes};
   use serde_json::json;

   trait BodyTest {
    fn as_str(&self) -> &str;
    }

    impl BodyTest for Bytes {
        fn as_str(&self) -> &str {
            std::str::from_utf8(self).unwrap()
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

       let resp = TestRequest::get().uri("/list/").send_request(&mut app).await;
       let body = to_bytes(resp.into_body()).await.unwrap();
       println!("{}", body.as_str());

       let resp = TestRequest::get().uri(&format!("/{}/", user.public_id)).send_request(&mut app).await;
       assert!(resp.status().is_success(), "Failed to find user");

       let user: User = test::read_body_json(resp).await;
       assert_eq!(user.email, "tore@cloudmaker.dev", "Found wrong user");

       let request_body = json!({
           "email": "tore@cloudmaker.dev",
           "password": "new",
       });

       let resp = TestRequest::put().uri(&format!("/users/{}", user.id)).set_json(&request_body).send_request(&mut app).await;
       assert!(resp.status().is_success(), "Failed to update user");

       let user: User = test::read_body_json(resp).await;
       assert_eq!("new", user.password, "Failed to change password for user");

       let resp = TestRequest::delete().uri(&format!("/users/{}", user.id)).send_request(&mut app).await;
       assert!(resp.status().is_success(), "Failed to delete user");

       let resp = TestRequest::get().uri(&format!("/users/{}", user.id)).send_request(&mut app).await;
       assert!(resp.status().is_client_error(), "It should not be possible to find the user after deletion");
   }
}