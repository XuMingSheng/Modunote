-- Add tab_order column to block_opens table
ALTER TABLE block_opens ADD COLUMN tab_order INTEGER;

-- Set initial tab_order values based on opened_at (oldest = 1, newest = highest)
UPDATE block_opens
SET tab_order = (
    SELECT COUNT(*)
    FROM block_opens b2
    WHERE b2.opened_at <= block_opens.opened_at
);

-- Make tab_order NOT NULL after setting values
-- SQLite doesn't support ALTER COLUMN, so we need to recreate the table
CREATE TABLE block_opens_new (
    block_id TEXT NOT NULL,
    opened_at TEXT NOT NULL,
    tab_order INTEGER NOT NULL,
    PRIMARY KEY (block_id),
    FOREIGN KEY (block_id) REFERENCES blocks (id) ON DELETE CASCADE
);

INSERT INTO block_opens_new SELECT block_id, opened_at, tab_order FROM block_opens;
DROP TABLE block_opens;
ALTER TABLE block_opens_new RENAME TO block_opens;

-- Recreate indexes if any exist
CREATE INDEX IF NOT EXISTS idx_block_opens_tab_order ON block_opens (tab_order);
