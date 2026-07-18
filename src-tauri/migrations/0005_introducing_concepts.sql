-- Migration 5: Major schema changes (English naming)
-- 1. Rename columns
ALTER TABLE accounts
RENAME COLUMN owner TO comment;

ALTER TABLE accounts
RENAME COLUMN description TO destination;

ALTER TABLE transactions
RENAME COLUMN description TO comment;

-- 2. Create new Concepts table
CREATE TABLE
  IF NOT EXISTS concepts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    type TEXT NOT NULL CHECK (type IN ('deposit', 'withdrawal', 'transfer')),
    name TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (account_id, type, name)
  );

-- Indexes for concepts
CREATE INDEX IF NOT EXISTS idx_concepts_account_type ON concepts (account_id, type);

CREATE INDEX IF NOT EXISTS idx_concepts_account_id ON concepts (account_id);

-- Trigger for updated_at
CREATE TRIGGER IF NOT EXISTS trg_concepts_updated_at AFTER
UPDATE ON concepts FOR EACH ROW BEGIN
UPDATE concepts
SET
  updated_at = CURRENT_TIMESTAMP
WHERE
  id = OLD.id;

END;

-- 3. Junction table for many-to-many relationship (transactions <-> concepts)
CREATE TABLE
  IF NOT EXISTS transaction_concepts (
    transaction_id INTEGER NOT NULL REFERENCES transactions (id) ON DELETE CASCADE,
    concept_id INTEGER NOT NULL REFERENCES concepts (id) ON DELETE RESTRICT,
    PRIMARY KEY (transaction_id, concept_id)
  );

-- Indexes for junction table
CREATE INDEX IF NOT EXISTS idx_transaction_concepts_tx ON transaction_concepts (transaction_id);

CREATE INDEX IF NOT EXISTS idx_transaction_concepts_concept ON transaction_concepts (concept_id);