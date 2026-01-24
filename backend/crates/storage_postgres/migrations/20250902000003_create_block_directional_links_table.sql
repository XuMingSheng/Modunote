CREATE TABLE IF NOT EXISTS block_directional_links (
    id UUID PRIMARY KEY NOT NULL,
    block_from_id UUID NOT NULL,
    block_to_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (block_from_id) REFERENCES blocks (id) ON DELETE CASCADE,
    FOREIGN KEY (block_to_id) REFERENCES blocks (id) ON DELETE CASCADE,
    UNIQUE(block_from_id, block_to_id)
);

CREATE INDEX IF NOT EXISTS idx_block_directional_links_block_from_id
    ON block_directional_links (block_from_id);
CREATE INDEX IF NOT EXISTS idx_block_directional_links_block_to_id
    ON block_directional_links (block_to_id);
