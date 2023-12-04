ALTER TABLE wallet RENAME COLUMN stripe_payment_method_id TO payment_method_id;
ALTER TABLE wallet ADD COLUMN wallet_card_attempt_id INT NOT NULL REFERENCES wallet_card_attempt(id);