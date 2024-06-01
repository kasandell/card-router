DROP TABLE IF EXISTS successful_end_to_end_charge;
DROP TABLE IF EXISTS wallet_card_charge_response;
DROP TABLE IF EXISTS wallet_card_charge;
DROP TABLE IF EXISTS expected_wallet_charge_reference;
DROP TABLE IF EXISTS passthrough_card_charge;
DROP TABLE IF EXISTS settled_wallet_transaction_ledger;
DROP TABLE IF EXISTS pending_wallet_transaction_ledger;
DROP TABLE IF EXISTS settled_passthrough_card_transaction_ledger;
DROP TABLE IF EXISTS pending_passthrough_card_transaction_ledger;
DROP TABLE IF EXISTS registered_transaction_metadata;
DROP TABLE IF EXISTS registered_transaction;


CREATE TABLE IF NOT EXISTS registered_transactions(
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users(id),
    transaction_id UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4(),
    memo VARCHAR(255) NOT NULL,
    amount_cents INT NOT NULL,
    mcc VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS outer_charge_ledger(
    id SERIAL PRIMARY KEY,
    registered_transaction_id INT NOT NULL REFERENCES registered_transactions(id),
    user_id INT NOT NULL REFERENCES users(id),
    passthrough_card_id INT NOT NULL REFERENCES passthrough_card(id),
    amount_cents INT NOT NULL,
    status VARCHAR(255) NOT NULL,
    is_success BOOLEAN DEFAULT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
    );
-- outer charge should only happen once for a given registered txn id
CREATE UNIQUE INDEX IF NOT EXISTS outer_charge_success_txn ON outer_charge_ledger(registered_transaction_id);


CREATE TABLE IF NOT EXISTS inner_charge_ledger(
    id SERIAL PRIMARY KEY,
    registered_transaction_id INT NOT NULL REFERENCES registered_transactions(id),
    user_id INT NOT NULL REFERENCES users(id),
    wallet_card_id INT NOT NULL REFERENCES wallet(id),
    amount_cents INT NOT NULL,
    status VARCHAR(255) NOT NULL,
    is_success BOOLEAN DEFAULT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    rule_id INTEGER DEFAULT NULL REFERENCES rule(id)
);
-- inner charge can happen several times, as long as not success
CREATE UNIQUE INDEX IF NOT EXISTS inner_charge_success_txn ON inner_charge_ledger(registered_transaction_id, is_success);

CREATE TABLE IF NOT EXISTS transaction_ledger(
    id SERIAL PRIMARY KEY,
    registered_transaction_id INT NOT NULL REFERENCES registered_transactions(id),
    inner_charge_ledger_id INT NOT NULL REFERENCES inner_charge_ledger(id),
    outer_charge_ledger_id INT NOT NULL REFERENCES outer_charge_ledger(id),
    rule_id INTEGER DEFAULT NULL REFERENCES rule(id)
);

CREATE UNIQUE INDEX IF NOT EXISTS transaction_ledger_registered_transaction_unique ON transaction_ledger(registered_transaction_id);
CREATE UNIQUE INDEX IF NOT EXISTS transaction_ledger_registered_inner_charge_unique ON transaction_ledger(inner_charge_ledger_id);
CREATE UNIQUE INDEX IF NOT EXISTS transaction_ledger_registered_outer_charge_unique ON transaction_ledger(outer_charge_ledger_id);

