DROP TABLE IF EXISTS transaction_ledger;
DROP TABLE IF EXISTS inner_charge_ledger;
DROP TABLE IF EXISTS outer_charge_ledger;
DROP TABLE IF EXISTS registered_transactions;

-- everything in the system is initiated with a registered transaction
CREATE TABLE IF NOT EXISTS registered_transaction(
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users(id),
    transaction_id UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4(),
    memo VARCHAR(255) NOT NULL,
    amount_cents INT NOT NULL,
    mcc VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);

CREATE TABLE IF NOT EXISTS registered_transaction_metadata(
    registered_transaction_id INT PRIMARY KEY NOT NULL REFERENCES registered_transaction(id),
    body TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);


CREATE TABLE IF NOT EXISTS pending_passthrough_card_transaction_ledger(
    id SERIAL PRIMARY KEY,
    registered_transaction_id INT NOT NULL REFERENCES registered_transaction(id),
    user_id INT NOT NULL REFERENCES users(id),
    passthrough_card_id INT NOT NULL REFERENCES passthrough_card(id),
    money_movement_direction VARCHAR(20) NOT NULL,
    money_movement_type VARCHAR(40) NOT NULL,
    amount_cents INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);

CREATE TABLE IF NOT EXISTS settled_passthrough_card_transaction_ledger(
    id SERIAL PRIMARY KEY,
    registered_transaction_id INT NOT NULL REFERENCES registered_transaction(id),
    user_id INT NOT NULL REFERENCES users(id),
    passthrough_card_id INT NOT NULL REFERENCES passthrough_card(id),
    money_movement_direction VARCHAR(20) NOT NULL,
    money_movement_type VARCHAR(40) NOT NULL,
    amount_cents INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);


CREATE TABLE IF NOT EXISTS pending_wallet_transaction_ledger(
    id SERIAL PRIMARY KEY,
    registered_transaction_id INT NOT NULL REFERENCES registered_transaction(id),
    user_id INT NOT NULL REFERENCES users(id),
    wallet_id INT NOT NULL REFERENCES wallet(id),
    money_movement_direction VARCHAR(20) NOT NULL,
    money_movement_type VARCHAR(40) NOT NULL,
    amount_cents INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);

CREATE TABLE IF NOT EXISTS settled_wallet_transaction_ledger(
    id SERIAL PRIMARY KEY,
    registered_transaction_id INT NOT NULL REFERENCES registered_transaction(id),
    user_id INT NOT NULL REFERENCES users(id),
    wallet_id INT NOT NULL REFERENCES wallet(id),
    money_movement_direction VARCHAR(20) NOT NULL,
    money_movement_type VARCHAR(40) NOT NULL,
    amount_cents INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);


-- sits in charge schema effectively, but tracks what we register against the outer card txn
CREATE TABLE IF NOT EXISTS passthrough_card_charge(
    id SERIAL PRIMARY KEY,
    registered_transaction_id INT NOT NULL REFERENCES registered_transaction(id),
    user_id INT NOT NULL REFERENCES users(id),
    passthrough_card_id INT NOT NULL REFERENCES passthrough_card(id),
    amount_cents INT NOT NULL,
    status VARCHAR(255) NOT NULL,
    is_success BOOLEAN DEFAULT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
    );
-- outer charge should only happen once for a given registered txn id
CREATE UNIQUE INDEX IF NOT EXISTS passthrough_card_success_txn ON passthrough_card_charge(registered_transaction_id);



CREATE TABLE IF NOT EXISTS expected_wallet_charge_reference(
    id SERIAL PRIMARY KEY,
    registered_transaction_id INT NOT NULL REFERENCES registered_transaction(id),
    reference_id UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4(),
    user_id INT NOT NULL REFERENCES users(id),
    wallet_card_id INT NOT NULL REFERENCES wallet(id),
    amount_cents INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);

CREATE TABLE IF NOT EXISTS wallet_card_charge(
    id SERIAL PRIMARY KEY,
    registered_transaction_id INT NOT NULL REFERENCES registered_transaction(id),
    user_id INT NOT NULL REFERENCES users(id),
    wallet_card_id INT NOT NULL REFERENCES wallet(id),
    amount_cents INT NOT NULL,
    rule_id INTEGER DEFAULT NULL REFERENCES rule(id),
    expected_wallet_charge_reference_id INT UNIQUE NOT NULL REFERENCES expected_wallet_charge_reference(id),
    resolved_charge_status VARCHAR(255) NOT NULL,
    psp_reference VARCHAR(255) UNIQUE,
    returned_reference VARCHAR(255) UNIQUE,
    returned_charge_status VARCHAR(30),
    is_success BOOLEAN DEFAULT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);
CREATE UNIQUE INDEX IF NOT EXISTS wallet_card_charge_success_txn ON wallet_card_charge(registered_transaction_id, is_success);

-- inner charge can happen several times, as long as not success
CREATE TABLE IF NOT EXISTS successful_end_to_end_charge(
    id SERIAL PRIMARY KEY,
    registered_transaction_id INT NOT NULL REFERENCES registered_transaction(id),
    wallet_card_charge_id INT NOT NULL REFERENCES wallet_card_charge(id),
    passthrough_card_charge_id INT NOT NULL REFERENCES passthrough_card_charge(id)
);

CREATE UNIQUE INDEX IF NOT EXISTS e2e_charge_registered_transaction_unique ON successful_end_to_end_charge(registered_transaction_id);
CREATE UNIQUE INDEX IF NOT EXISTS e2e_charge_registered_wallet_card_charge_unique ON successful_end_to_end_charge(wallet_card_charge_id);
CREATE UNIQUE INDEX IF NOT EXISTS e2e_charge_registered_passthrough_card_charge_unique ON successful_end_to_end_charge(passthrough_card_charge_id);
