CREATE TABLE
    IF NOT EXISTS accounts (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        number TEXT NOT NULL UNIQUE,
        name TEXT NOT NULL,
        description TEXT,
        owner TEXT NOT NULL,
        balance REAL NOT NULL DEFAULT 0,
        currency TEXT NOT NULL,
        created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
    );

CREATE TRIGGER IF NOT EXISTS trg_accounts_updated_at AFTER
UPDATE ON accounts FOR EACH ROW BEGIN
UPDATE accounts
SET
    updated_at = CURRENT_TIMESTAMP
WHERE
    id = OLD.id;

END;