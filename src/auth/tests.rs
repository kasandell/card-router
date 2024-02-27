#[cfg(test)]
mod tests {
    use actix_web::{App, test, get, HttpResponse, web};
    use actix_web::test::TestRequest;
    use chrono::NaiveDate;
    use jsonwebtoken::{Algorithm, decode, DecodingKey, Validation};
    use serde_json::json;
    use uuid::Uuid;
    use crate::api_error::ApiError;
    use crate::user::entity::{User, UserMessage};
    use crate::test_helper::initialize_user;
    use crate::auth::config::config;
    use crate::auth::constant::JWT_SECRET;
    use crate::auth::entity::Claims;
    use crate::auth::response::LoginResponse;
    use std::str::FromStr;
    use reqwest::Response;

    #[actix_web::test]
    async fn test_login() {
        crate::test::init();
        let user = initialize_user();
        println!("initialized user");
        let request_body = json!({
            "email": "test@example.com",
            "password": "1234",
        });

        let mut app = test::init_service(App::new().configure(config)).await;
        let resp = TestRequest::post().uri("/login/").set_json(&request_body).send_request(&mut app).await;
        println!("{:?}", resp);
        assert!(resp.status().is_success(), "Failed to create user");
        let jwt: LoginResponse = test::read_body_json(resp).await;
        println!("{:?}", jwt);
        assert_ne!(jwt.token, "".to_string());
        let claims = decode::<Claims>(
            &jwt.token,
            &DecodingKey::from_secret(JWT_SECRET),
            &Validation::new(Algorithm::HS512),
        )
            .map_err(|_| ApiError::new(401, "JWT error".to_string())).expect("should decode");

        let id = Uuid::from_str(&claims.claims.sub).expect("needs id");
        assert_eq!(id, user.public_id);

        user.delete_self().expect("user should delete");
        assert!(User::find(&user.public_id).is_err())
    }

    #[actix_web::test]
    async fn test_login_fails() {
        crate::test::init();
        let user = initialize_user();
        let request_body = json!({
            "email": "test@example.com",
            "password": "1235",
        });

        let mut app = test::init_service(App::new().configure(config)).await;
        let resp = TestRequest::post().uri("/login/").set_json(&request_body).send_request(&mut app).await;
        println!("{:?}", resp);
        assert!(!resp.status().is_success(), "Login should fail");
        assert_eq!(401, resp.status().as_u16());
        user.delete_self().expect("user should delete");
        assert!(User::find(&user.public_id).is_err())
    }

    #[actix_web::test]
    async fn test_middleware() {
        crate::test::init();
        println!("init test");
        let user = initialize_user();
        println!("init user");
        let request_body = json!({
            "email": "test@example.com",
            "password": "1234",
        });

        let mut app = test::init_service(
            App::new()
                .service(crate::auth::controller::login)
                .service(
                    web::scope("")
                        .wrap(crate::middleware::auth::Auth)
                        .service(test_get)
                )
        ).await;
        let resp = TestRequest::post().uri("/login/").set_json(&request_body).send_request(&mut app).await;
        println!("{:?}", resp);
        assert!(resp.status().is_success(), "Failed to create user");
        let jwt: LoginResponse = test::read_body_json(resp).await;
        let mut get_resp = TestRequest::get().uri("/test/get/")
            .insert_header(
                ("Authorization", format!("Bearer {0}", jwt.token))
            )
            .send_request(&mut app).await;

        assert!(get_resp.status().is_success());

        get_resp = TestRequest::get().uri("/test/get/")
            .send_request(&mut app).await;

        assert!(!get_resp.status().is_success());

        user.delete_self().expect("user should delete");
        assert!(User::find(&user.public_id).is_err())
    }


    #[get("/test/get/")]
    async fn test_get() -> Result<HttpResponse, ApiError> {
        Ok(
            HttpResponse::Ok().finish()
        )
    }
}