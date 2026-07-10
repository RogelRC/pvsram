CREATE TABLE IF NOT EXISTS transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    type TEXT NOT NULL CHECK (type IN ('deposit', 'withdrawal', 'transfer')),
    account_id INTEGER NOT NULL REFERENCES accounts(id),
    related_account_id INTEGER REFERENCES accounts(id),
    amount REAL NOT NULL CHECK (amount > 0),
    description TEXT,
    occurred_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (
        (type != 'transfer' AND related_account_id IS NULL)
        OR
        (type = 'transfer' AND related_account_id IS NOT NULL AND related_account_id != account_id)
    )
);

CREATE INDEX IF NOT EXISTS idx_transactions_account_id ON transactions(account_id);
CREATE INDEX IF NOT EXISTS idx_transactions_related_account_id ON transactions(related_account_id);
CREATE INDEX IF NOT EXISTS idx_transactions_occurred_at ON transactions(occurred_at);

CREATE TRIGGER IF NOT EXISTS trg_transactions_updated_at
AFTER UPDATE ON transactions
FOR EACH ROW
BEGIN
    UPDATE transactions SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;

ALTER TABLE accounts DROP COLUMN balance;
