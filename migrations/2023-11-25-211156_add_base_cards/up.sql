-- Your SQL goes here
INSERT INTO credit_card_issuer(id, "name") VALUES (1, 'Chase'), (2, 'Bilt'), (3, 'CapitalOne');
INSERT INTO credit_card_type(id, "name") VALUES (1, 'Visa'), (2, 'MasterCard'), (3, 'American Express');
INSERT INTO 
    credit_card("name", credit_card_type_id, credit_card_issuer_id, card_image_url)
VALUES
    ('Sapphire Preferred', 1, 1, 'https://creditcards.chase.com/K-Marketplace/images/cardart/sapphire_preferred_card.png'),
    ('Sapphire Reserve', 1, 1, 'https://creditcards.chase.com/K-Marketplace/images/cardart/sapphire_reserve_card.png'),
    ('World Elite', 2, 2, 'https://creditcards.wellsfargo.com/W-Card-MarketPlace/v11-14-23/images/Products/Bilt/Bilt_card_D.png');