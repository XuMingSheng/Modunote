CREATE TABLE IF NOT EXISTS block_pins (
    block_id UUID PRIMARY KEY NOT NULL,
    pinned_at TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (block_id) REFERENCES blocks (id)
        ON DELETE CASCADE
);
