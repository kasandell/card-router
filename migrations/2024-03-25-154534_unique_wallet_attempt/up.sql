ALTER TABLE wallet
    ADD CONSTRAINT unique_wallet_attempt_id UNIQUE (wallet_card_attempt_id);