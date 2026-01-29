CREATE TABLE IF NOT EXISTS blocks (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL DEFAULT '',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS block_opens (
    block_id TEXT PRIMARY KEY NOT NULL,
    opened_at TEXT NOT NULL,
    FOREIGN KEY (block_id) REFERENCES blocks (id) 
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS block_pins (
    block_id TEXT PRIMARY KEY NOT NULL,
    pinned_at TEXT NOT NULL,
    FOREIGN KEY (block_id) REFERENCES blocks (id) 
        ON DELETE CASCADE
);




