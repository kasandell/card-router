ALTER TABLE wallet
ADD COLUMN credit_card_id INT NOT NULL REFERENCES credit_card(id);