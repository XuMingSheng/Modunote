CREATE TABLE IF NOT EXISTS block_opens (
    block_id UUID PRIMARY KEY NOT NULL,
    opened_at TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (block_id) REFERENCES blocks (id)
        ON DELETE CASCADE
);
