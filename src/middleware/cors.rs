use actix_cors::Cors;
use actix_web::http::{header, Method};

pub fn cors(client_origin_url: &str) -> Cors {
    Cors::default()
        .allowed_origin(client_origin_url)
        .allowed_methods([Method::GET])
        .allowed_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .max_age(86_400)
}

#[cfg(test)]
mod test {
    use crate::middleware::cors::cors;

    #[test]
    fn test_cors_creates() {
        let returned_cors = cors("localhost:3000");
    }
}