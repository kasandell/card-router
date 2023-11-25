CREATE TABLE IF NOT EXISTS credit_card_issuer (
    id SERIAL PRIMARY KEY,
    public_id UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4(),
    "name" VARCHAR(255) UNIQUE NOT NULL, -- Bilt, Chase, Etc.
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);

CREATE TABLE IF NOT EXISTS credit_card_type (
    id SERIAL PRIMARY KEY,
    public_id UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4(),
    "name" VARCHAR(255) UNIQUE NOT NULL, -- Visa, Mastercard, Amex
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);

CREATE TABLE IF NOT EXISTS credit_card (
    id SERIAL PRIMARY KEY,
    public_id UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4(),
    "name" VARCHAR(255) UNIQUE NOT NULL, -- Sapphire Reserve, etc.,
    credit_card_type_id INT NOT NULL REFERENCES credit_card_type(id),
    credit_card_issuer_id INT NOT NULL REFERENCES credit_card_issuer(id),
    card_image_url VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);

CREATE UNIQUE INDEX IF NOT EXISTS unique_card_name_issuer_type ON credit_card("name", credit_card_type_id, credit_card_issuer_id);