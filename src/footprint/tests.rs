#[cfg(test)]
mod tests {
    use footprint::models::CreateClientTokenRequest;
    use crate::footprint::service::{FootprintService, FootprintServiceTrait};
    use crate::test_helper::user::create_mock_user;
    use crate::footprint::helper::{card_request_parts_for_card_id, get_scopes_for_request};
    use std::sync::Arc;
    use uuid::Uuid;
    use crate::footprint::request::ChargeThroughProxyRequest;
    use crate::test_helper::general::init;

    #[actix_web::test]
    pub async fn test_token() {
        init();
        let svc = Arc::new(FootprintService::new());
        let mut user = create_mock_user();
        user.footprint_vault_id = "dont show this in githhb".to_string();
        let card_id = "1234";
        let parts = card_request_parts_for_card_id(card_id).expect("create");
        let request = CreateClientTokenRequest {
            ttl: 110,
            scopes: get_scopes_for_request(),
            fields: parts
        };
        let resp = svc.clone().create_client_token(
            &user,
            request
        ).await.expect("should get token");
    }

    #[actix_web::test]
    pub async fn test_vaults_ok() {
        init();
        let svc = Arc::new(FootprintService::new());
        let mut user = create_mock_user();
        user.footprint_vault_id = "dont show this in github".to_string();
        let card_id = "test_card_id";
        let parts = card_request_parts_for_card_id(card_id).expect("create");
        let request = CreateClientTokenRequest {
            ttl: 110,
            scopes: get_scopes_for_request(),
            fields: parts
        };
        let resp = svc.clone().create_client_token(
            &user,
            request
        ).await.expect("should get token");
        println!("{}", resp.token);
    }

    #[actix_web::test]
    pub async fn test_proxy_jit() {
        init();
        let svc = Arc::new(FootprintService::new());
        let mut user = create_mock_user();
        user.footprint_vault_id = "dont show this in github".to_string();
        let res = svc.clone().proxy_adyen_payment_request(
            &ChargeThroughProxyRequest {
                amount_cents: 100,
                mcc: "7184",
                payment_method_id: "test_card_id",
                customer_public_id: &user.footprint_vault_id,
                idempotency_key: &Uuid::new_v4(),
                reference:  &Uuid::new_v4().to_string(),
                statement: "coffee",
            }
        ).await;
        match res {
            Ok(body) => {
                println!("{:?}", body);
            }
            Err(error) => {
                println!("{:?}", error);
            }
        }
    }

}