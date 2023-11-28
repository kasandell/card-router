CREATE TABLE IF NOT EXISTS rule (
    id SERIAL PRIMARY KEY,
    public_id UUID NOT NULL DEFAULT uuid_generate_v4(),
    credit_card_id INT NOT NULL REFERENCES credit_card(id),
    rule_mcc VARCHAR(4) REFERENCES mcc_mapping(mcc_code),
    merchant_name VARCHAR(255),
    recurring_day_of_month VARCHAR(255),
    "start_date" DATE,
    "end_date" DATE,
    rule_status VARCHAR(255) NOT NULL
);