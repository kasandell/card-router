CREATE TABLE IF NOT EXISTS wallet_card_attempt (
    id SERIAL PRIMARY KEY,
    public_id UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4(),
    user_id INT NOT NULL REFERENCES users(id),
    credit_card_id INT NOT NULL REFERENCES credit_card(id),
    expected_reference_id VARCHAR(255) UNIQUE NOT NULL, -- this is from when we create, and it is our internal
    psp_id VARCHAR(255), -- this is from when we match and this is from adyen
    "status" VARCHAR(255) NOT NULL DEFAULT 'PENDING', -- PENDING, MATCHED, FAILED,
    recurring_detail_reference VARCHAR(255) UNIQUE, -- this is the card detail and this is from adyen
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);