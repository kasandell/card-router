WITH credits AS (
    SELECT * FROM pending_wallet_transaction_ledger WHERE money_movement_direction = 'CREDIT'
), debits AS (
    SELECT * FROM pending_wallet_transaction_ledger WHERE money_movement_direction = 'DEBIT'
)
SELECT COALESCE(SUM(amount_cents), 0) FROM credits
UNION ALL
SELECT COALESCE(SUM(amount_cents), 0) FROM debits;