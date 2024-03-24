SELECT setval(pg_get_serial_sequence('rule', 'id'), max(id)) FROM rule;
SELECT setval(pg_get_serial_sequence('category', 'id'), max(id)) FROM category;
SELECT setval(pg_get_serial_sequence('mcc_mapping', 'id'), max(id)) FROM mcc_mapping;
SELECT setval(pg_get_serial_sequence('credit_card', 'id'), max(id)) FROM credit_card;
SELECT setval(pg_get_serial_sequence('credit_card_type', 'id'), max(id)) FROM credit_card_type;
SELECT setval(pg_get_serial_sequence('credit_card_issuer', 'id'), max(id)) FROM credit_card_issuer;
