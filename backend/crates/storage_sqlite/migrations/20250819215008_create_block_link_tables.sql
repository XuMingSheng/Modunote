-- Migration: create_block_link_tables
-- Create separate tables for directional and related block links

-- Create block_directional_links table for directional relationships
CREATE TABLE IF NOT EXISTS block_directional_links (
    id TEXT PRIMARY KEY NOT NULL,
    block_from_id TEXT NOT NULL,
    block_to_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (block_from_id) REFERENCES blocks (id) ON DELETE CASCADE,
    FOREIGN KEY (block_to_id) REFERENCES blocks (id) ON DELETE CASCADE,
    UNIQUE(block_from_id, block_to_id)
);
-- Create block_related_links table for bidirectional relationships
CREATE TABLE IF NOT EXISTS block_related_links (
    id TEXT PRIMARY KEY NOT NULL,
    block_a_id TEXT NOT NULL,
    block_b_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (block_a_id) REFERENCES blocks (id) ON DELETE CASCADE,
    FOREIGN KEY (block_b_id) REFERENCES blocks (id) ON DELETE CASCADE,
    UNIQUE(block_a_id, block_b_id),
    CHECK(block_a_id < block_b_id)  -- Enforce canonical ordering
);

-- Create indexes for efficient querying
CREATE INDEX idx_block_directional_links_block_from_id ON block_directional_links (block_from_id);
CREATE INDEX idx_block_directional_links_block_to_id ON block_directional_links (block_to_id);
CREATE INDEX idx_block_related_links_block_a_id ON block_related_links (block_a_id);
CREATE INDEX idx_block_related_links_block_b_id ON block_related_links (block_b_id);
