INSERT INTO  rule(
    id,
    credit_card_id,
    rule_category_id,
    points_multiplier
) VALUES
      (13, 3, 1, 2),
      (14, 3, 2, 2),
      (15, 3, 3, 2),
      (16, 3, 4, 3);


INSERT INTO  rule(
    id,
    credit_card_id,
    rule_category_id,
    points_multiplier,
    recurring_day_of_month
) VALUES
      (17, 3, 1, 4, 'FIRST'),
      (18, 3, 2, 4, 'FIRST'),
      (19, 3, 3, 4, 'FIRST'),
      (20, 3, 4, 6, 'FIRST');