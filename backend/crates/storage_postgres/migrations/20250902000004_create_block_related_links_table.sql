CREATE TABLE IF NOT EXISTS block_related_links (
    id UUID PRIMARY KEY NOT NULL,
    block_a_id UUID NOT NULL,
    block_b_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (block_a_id) REFERENCES blocks (id) ON DELETE CASCADE,
    FOREIGN KEY (block_b_id) REFERENCES blocks (id) ON DELETE CASCADE,
    UNIQUE(block_a_id, block_b_id),
    CHECK(block_a_id < block_b_id)
);

CREATE INDEX IF NOT EXISTS idx_block_related_links_block_a_id
    ON block_related_links (block_a_id);
CREATE INDEX IF NOT EXISTS idx_block_related_links_block_b_id
    ON block_related_links (block_b_id);
