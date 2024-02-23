# Rust specific 
https://github.com/serde-rs/serde/issues/1019

# Authenitication 
https://auth0.com/blog/build-an-api-in-rust-with-jwt-authentication-using-actix-web/
https://github.com/actix/examples/blob/master/middleware/various/src/redirect.rs
https://github.com/SakaDream/actix-web-rest-api-with-jwt/blob/master/src/middleware/auth_middleware.rs
https://blog.logrocket.com/jwt-authentication-in-rust/

# Architecture
https://cloudmaker.dev/how-to-create-a-rest-api-in-rust/
https://cloudmaker.dev/actix-integration-tests/

# Lithic 
https://github.com/lithic-com/asa-demo-python/blob/main/webhook/authorization.py
https://raw.githubusercontent.com/lithic-com/lithic-openapi/main/lithic-openapi.yml <- generate from this
run `yq -o=json lithic-openapi.yml -P > lithic-openapi.json`
then generate from json (and fix errors)

# Code generation
```
openapi-generator generate -g rust \
-i json/PaymentService-v68.json \
-o clients/payment \
--additional-properties="packageName=adyen-payment"
```

# Adyen
https://docs.adyen.com/online-payments/tokenization/create-and-use-tokens/#test-and-go-live
https://docs.adyen.com/development-resources/testing/test-card-numbers/
https://github.com/Adyen/adyen-openapi
https://docs.adyen.com/api-explorer/Checkout/69/post/payments/_paymentPspReference_/cancels
going to need to modify wallet architecture -> add card attempt or something similar, then when we see a webhook come in with matching ref number, we tie to that


# Testing
Run with `cargo test -- --nocapture --test-threads=1`
https://danielbunte.medium.com/a-guide-to-testing-and-mocking-in-rust-a73d022b4075

# Env
DATABASE_URL=""
ADYEN_API_KEY=""
ADYEN_MERCHANT_ACCOUNT_NAME=""
AUTHORITY=
LITHIC_API_KEY=
MODE=
LITHIC_WEBHOOK_URL=

# TODOs:
Restructure data in such a way that we can mock it. ie without having to actually hit db for tests

fix openapi bug: 
in patch card
line 618
let local_var_uri_str = format!("{}/cards/{card_token}", local_var_configuration.base_path,
    card_token=card_token.as_str().ok_or(Error::Io(io::Error::new(Other, "")))?
);


FOR WALLET MATCHING TO WORK PROPERLY
frontend needs two screens, the first one is where we select card type. this needs to call backend to create wallet card attempt
do not present next screen until this is ready. 
then present adyen screen. this guarantees that we have a wallet card attempt to match against on backend