-- CREATE TABLE IF NOT EXISTS passthrough_card_status (
--     id SERIAL PRIMARY KEY,
--     "status" VARCHAR(255) UNIQUE NOT NULL,
--     created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
--     updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
-- );
-- INSERT INTO passthrough_card_status (id, "status") VALUES 
--     (1, 'CLOSED'), 
--     (2, 'OPEN'),
--     (3, 'PAUSED'),
--     (4, 'PENDING_ACTIVATION'),
--     (5, 'PENDING_FULFILLMENT');
-- 
-- CREATE TABLE IF NOT EXISTS passthrough_card_type (
--     id SERIAL PRIMARY KEY,
--     "type" VARCHAR(255) UNIQUE NOT NULL,
--     created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
--     updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
-- );
-- INSERT INTO passthrough_card_type(id, "type") VALUES
--     (1, 'VIRTUAL'),
--     (2, 'PHYSICAL'),
--     (3, 'SINGLE_USE'),
--     (4, 'MERCHANT_LOCKED');

CREATE TABLE IF NOT EXISTS passthrough_card (
    id SERIAL PRIMARY KEY,
    public_id UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4(),
    passthrough_card_status VARCHAR(255) NOT NULL,-- _id INT NOT NULL REFERENCES passthrough_card_status(id) DEFAULT 1,
    is_active BOOLEAN DEFAULT NULL,
    user_id INT NOT NULL REFERENCES users(id),
    token VARCHAR(255) UNIQUE NOT NULL, -- lithic token representing this card
    expiration DATE NOT NULL,
    last_four VARCHAR(4) NOT NULL, -- last 4 of the card
    passthrough_card_type VARCHAR(255) NOT NULL,-- _id INT NOT NULL REFERENCES passthrough_card_type(id),
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);

CREATE UNIQUE INDEX IF NOT EXISTS user_passthrough_active ON passthrough_card(user_id, is_active);
CREATE INDEX IF NOT EXISTS passthrough_card_status_idx ON passthrough_card(passthrough_card_status);
CREATE INDEX IF NOT EXISTS passthrough_card_type_idx ON passthrough_card(passthrough_card_type);