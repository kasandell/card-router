ALTER TABLE wallet_card_attempt
    ADD COLUMN psp_id VARCHAR(255),
    ADD COLUMN recurring_detail_reference VARCHAR(255) UNIQUE;
