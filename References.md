# Rust specific 
https://github.com/serde-rs/serde/issues/1019

# Coverage 
```
cargo tarpaulin --out Html -- --nocapture --test-threads=1

cargo tarpaulin --engine=llvm --out Html --exclude-files "src/adyen/*" --exclude-files "src/lithic/*" --exclude-files "src/*/constant.rs" --exclude-files "src/*/config.rs" --exclude-files "src/schema.rs" -- --nocapture --test-threads=1
```


# Logging
https://mcarton.github.io/rust-derivative/latest/Debug.html


# Timing 
https://crates.io/crates/metered

# Actix

https://github.com/actix/examples/blob/344bcfce/middleware/middleware/src/read_request_body.rs
https://github.com/actix/actix-extras/blob/master/actix-redis/src/redis.rs


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
https://docs.adyen.com/development-resources/error-codes/
https://dev.to/adyen/use-adyen-tokenization-to-implement-subscriptions-in-net-4fkl


# Testing
Run with `cargo test -- --nocapture --test-threads=1`
https://danielbunte.medium.com/a-guide-to-testing-and-mocking-in-rust-a73d022b4075

# Env
```
DATABASE_URL=
ADYEN_API_KEY_2=
ADYEN_API_KEY=
ADYEN_MERCHANT_ACCOUNT_NAME=
AUTHORITY=
LITHIC_API_KEY=
MODE=
LITHIC_WEBHOOK_URL=
AUTH0_AUDIENCE=
AUTH0_DOMAIN=
CLIENT_ORIGIN_URL=
FOOTPRINT_VAULT_PROXY_ID=
FOOTPRINT_SECRET_KEY_2=
FOOTPRINT_SECRET_KEY=
OTEL_EXPORTER_OTLP_ENDPOINT=
OTEL_EXPORTER_OTLP_HEADERS=
OTEL_EXPORTER_OTLP_PROTOCOL=
OTEL_SERVICE_NAME=
RUST_LOG=
```

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


openapi anyof is forcing some of the api parameters to show as required even though they're not and we're getting 
bad request from it. need to make a custom api schema probably.