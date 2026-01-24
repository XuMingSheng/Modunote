CREATE TABLE IF NOT EXISTS block_directional_paths (
    id UUID PRIMARY KEY NOT NULL,
    block_ancestor_id UUID NOT NULL,
    block_descendant_id UUID NOT NULL,
    block_path_ids UUID[] NOT NULL,
    path_length INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (block_ancestor_id) REFERENCES blocks (id) ON DELETE CASCADE,
    FOREIGN KEY (block_descendant_id) REFERENCES blocks (id) ON DELETE CASCADE,
    UNIQUE(block_path_ids)
);

CREATE INDEX IF NOT EXISTS idx_block_directional_paths_ancestor
    ON block_directional_paths (block_ancestor_id);
CREATE INDEX IF NOT EXISTS idx_block_directional_paths_descendant
    ON block_directional_paths (block_descendant_id);
CREATE INDEX IF NOT EXISTS idx_block_directional_paths_ancestor_length
    ON block_directional_paths (block_ancestor_id, path_length);
CREATE INDEX IF NOT EXISTS idx_block_directional_paths_descendant_length
    ON block_directional_paths (block_descendant_id, path_length);
