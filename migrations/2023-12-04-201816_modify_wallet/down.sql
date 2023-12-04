ALTER TABLE wallet RENAME COLUMN payment_method_id TO stripe_payment_method_id;
ALTER TABLE wallet DROP COLUMN wallet_card_attempt_id;