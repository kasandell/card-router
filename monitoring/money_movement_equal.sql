WITH pc_credits AS (
    SELECT COALESCE(SUM(amount_cents), 0) AS amount FROM settled_passthrough_card_transaction_ledger WHERE money_movement_direction = 'CREDIT'
), pc_debits AS (
    SELECT COALESCE(SUM(amount_cents), 0) AS amount FROM settled_passthrough_card_transaction_ledger WHERE money_movement_direction = 'DEBIT'
), wc_credits AS (
    SELECT COALESCE(SUM(amount_cents), 0) AS amount FROM settled_wallet_transaction_ledger WHERE money_movement_direction = 'CREDIT'
), wc_debits AS (
    SELECT COALESCE(SUM(amount_cents), 0) AS amount FROM settled_wallet_transaction_ledger WHERE money_movement_direction = 'DEBIT'
), wc_balance AS (
    SELECT (SELECT amount FROM wc_debits LIMIT 1) - (SELECT amount FROM wc_credits LIMIT 1) as balance
), pc_balance AS (
      SELECT (SELECT amount FROM pc_debits LIMIT 1) - (SELECT amount FROM pc_credits LIMIT 1) as balance
)
SELECT balance FROM wc_balance
UNION ALL
SELECT balance FROM pc_balance;
