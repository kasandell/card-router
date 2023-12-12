# Architecture
https://cloudmaker.dev/how-to-create-a-rest-api-in-rust/
https://cloudmaker.dev/actix-integration-tests/

# Lithic 
https://github.com/lithic-com/asa-demo-python/blob/main/webhook/authorization.py

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
