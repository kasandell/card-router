#[cfg(test)]
mod tests {
    use adyen_checkout::models::payment_response::ResultCode;
    use adyen_checkout::models::PaymentResponse;
    use adyen_webhooks::models::{Amount, RecurringContractNotificationRequestItem, RecurringContractNotificationRequestItemWrapper};
    use adyen_webhooks::models::recurring_contract_notification_request_item::EventCode;
    use crate::wallet::entity::{InsertableCardAttempt, Wallet, WalletCardAttempt};
    use crate::test_helper::initialize_user;
    use crate::wallet::constant::WalletCardAttemptStatus;
    use crate::webhooks::adyen_handler::AdyenHandler;
    use crate::charge_engine::engine::Engine as ChargeEngine;
    use crate::adyen_service::checkout::service::MockAdyenChargeServiceTrait;
    use crate::api_error::ApiError;
    use crate::asa::request::{AsaRequest, Avs, Card, CardholderAuthentication, create_example_asa, Event, Funding, Merchant, POS, TokenInfo};
    use crate::asa::response::{AsaResponse, AsaResponseResult};
    use crate::rule_engine::engine::{MockRuleEngineTrait, RuleEngineTrait};
    use crate::user::entity::User;
    use crate::webhooks::lithic_handler::LithicHandler;
    use std::default::Default;

    #[actix_web::test]
    pub async fn test_handle() {
        let mut charge_service = MockAdyenChargeServiceTrait::new();
        let mut rule_engine = MockRuleEngineTrait::new();


        let user_id = 1;
        let amount_cents = 100;
        let mcc = "7184";
        let statement = "test statement";
        let payment_method_1 = "card_123";
        let payment_method_2 = "card_246";

        let mut user = User::create_test_user(
            user_id
        );
        user.public_id = Default::default();
        let mut card_1 = Wallet::create_test_wallet(
            1,
            user_id,
            1,
        );
        card_1.payment_method_id = payment_method_1.to_string();

        let mut card_2 = Wallet::create_test_wallet(
            2,
            user_id,
            2
        );
        card_2.payment_method_id = payment_method_2.to_string();


        let psp_ref = "abc123".to_string();
        let psp_ref_2  = "abc125".to_string();
        let mut resp_1 = PaymentResponse::new();
        resp_1.result_code = Some(ResultCode::Refused);
        resp_1.psp_reference = Some(psp_ref.clone());
        let mut resp_2 = PaymentResponse::new();
        resp_2.result_code = Some(ResultCode::Authorised);
        resp_2.psp_reference = Some(psp_ref_2.clone());

        let cards = vec![card_1, card_2];

        rule_engine.expect_order_user_cards_for_request()
            .times(1)
            .return_const(
                Ok(
                    cards
                )
            );

        charge_service.expect_charge_card_on_file()
            .withf(
                move |charge_request| {
                    charge_request.amount_cents == amount_cents
                        && charge_request.mcc == mcc
                        && charge_request.payment_method_id == payment_method_1.to_string()
                        && charge_request.customer_public_id == user.public_id
                }
            )
            .times(1)
            .return_const(
                Ok(
                    resp_1
                )
            );

        charge_service.expect_charge_card_on_file()
            .withf(
                move |charge_request| {
                    charge_request.amount_cents == amount_cents
                        && charge_request.mcc == mcc
                        && charge_request.payment_method_id == payment_method_2.to_string()
                        && charge_request.customer_public_id == user.public_id
                }
            )
            .times(1)
            .return_const(
                Ok(
                    resp_2
                )
            );

        let handler = LithicHandler::new_with_engines(
            Box::new(charge_service),
            Box::new(rule_engine)
        );

        let asa = create_example_asa(
            amount_cents,
            mcc.to_string()
        );

        let res = handler.handle(
            asa.clone()
        ).await.expect("no error");
        assert_eq!(AsaResponseResult::Approved, res.result);
        assert_eq!(asa.token, res.token);
    }

}