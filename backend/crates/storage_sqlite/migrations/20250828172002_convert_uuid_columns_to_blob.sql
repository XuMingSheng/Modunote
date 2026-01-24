-- Migration: convert_uuid_columns_to_blob
-- Convert all UUID columns from TEXT to BLOB to match sqlx's binary storage

-- 1. blocks table
ALTER TABLE blocks RENAME TO blocks_old;

CREATE TABLE blocks (
    id BLOB PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

INSERT INTO blocks (id, title, content, created_at, updated_at)
SELECT id, title, content, created_at, updated_at FROM blocks_old;

DROP TABLE blocks_old;

-- 2. block_opens table  
ALTER TABLE block_opens RENAME TO block_opens_old;

CREATE TABLE block_opens (
    block_id BLOB PRIMARY KEY NOT NULL,
    opened_at TEXT NOT NULL,
    tab_order INTEGER NOT NULL,
    FOREIGN KEY (block_id) REFERENCES blocks (id) 
        ON DELETE CASCADE
);

INSERT INTO block_opens (block_id, opened_at, tab_order)
SELECT block_id, opened_at, tab_order FROM block_opens_old;

DROP TABLE block_opens_old;

-- 3. block_pins table
ALTER TABLE block_pins RENAME TO block_pins_old;

CREATE TABLE block_pins (
    block_id BLOB PRIMARY KEY NOT NULL,
    pinned_at TEXT NOT NULL,
    FOREIGN KEY (block_id) REFERENCES blocks (id) 
        ON DELETE CASCADE
);

INSERT INTO block_pins (block_id, pinned_at)
SELECT block_id, pinned_at FROM block_pins_old;

DROP TABLE block_pins_old;

-- 4. block_directional_links table
ALTER TABLE block_directional_links RENAME TO block_directional_links_old;

CREATE TABLE block_directional_links (
    id BLOB PRIMARY KEY NOT NULL,
    block_from_id BLOB NOT NULL,
    block_to_id BLOB NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (block_from_id) REFERENCES blocks (id) ON DELETE CASCADE,
    FOREIGN KEY (block_to_id) REFERENCES blocks (id) ON DELETE CASCADE,
    UNIQUE(block_from_id, block_to_id)
);

INSERT INTO block_directional_links (id, block_from_id, block_to_id, created_at)
SELECT id, block_from_id, block_to_id, created_at FROM block_directional_links_old;

DROP TABLE block_directional_links_old;

-- Recreate indexes for block_directional_links
CREATE INDEX idx_block_directional_links_block_from_id ON block_directional_links (block_from_id);
CREATE INDEX idx_block_directional_links_block_to_id ON block_directional_links (block_to_id);

-- 5. block_related_links table
ALTER TABLE block_related_links RENAME TO block_related_links_old;

CREATE TABLE block_related_links (
    id BLOB PRIMARY KEY NOT NULL,
    block_a_id BLOB NOT NULL,
    block_b_id BLOB NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (block_a_id) REFERENCES blocks (id) ON DELETE CASCADE,
    FOREIGN KEY (block_b_id) REFERENCES blocks (id) ON DELETE CASCADE,
    UNIQUE(block_a_id, block_b_id),
    CHECK(block_a_id < block_b_id)  -- Enforce ordering
);

INSERT INTO block_related_links (id, block_a_id, block_b_id, created_at)
SELECT id, block_a_id, block_b_id, created_at FROM block_related_links_old;

DROP TABLE block_related_links_old;

-- Recreate indexes for block_related_links
CREATE INDEX idx_block_related_links_block_a_id ON block_related_links (block_a_id);
CREATE INDEX idx_block_related_links_block_b_id ON block_related_links (block_b_id);

-- 6. block_directional_paths table
ALTER TABLE block_directional_paths RENAME TO block_directional_paths_old;

CREATE TABLE block_directional_paths (
    id BLOB PRIMARY KEY NOT NULL,
    block_ancestor_id BLOB NOT NULL,
    block_descendant_id BLOB NOT NULL,
    block_path_ids TEXT NOT NULL, -- Keep as JSON array of UUID strings
    path_length INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (block_ancestor_id) REFERENCES blocks (id) ON DELETE CASCADE,
    FOREIGN KEY (block_descendant_id) REFERENCES blocks (id) ON DELETE CASCADE,
    UNIQUE(block_path_ids) -- Path itself is unique
);

INSERT INTO block_directional_paths (id, block_ancestor_id, block_descendant_id, block_path_ids, path_length, created_at)
SELECT id, block_ancestor_id, block_descendant_id, block_path_ids, path_length, created_at FROM block_directional_paths_old;

DROP TABLE block_directional_paths_old;

-- Recreate indexes for block_directional_paths
CREATE INDEX idx_block_directional_paths_ancestor ON block_directional_paths (block_ancestor_id);
CREATE INDEX idx_block_directional_paths_descendant ON block_directional_paths (block_descendant_id);

-- Composite indexes for common tree queries
CREATE INDEX idx_block_directional_paths_ancestor_length ON block_directional_paths (block_ancestor_id, path_length);
CREATE INDEX idx_block_directional_paths_descendant_length ON block_directional_paths (block_descendant_id, path_length);
