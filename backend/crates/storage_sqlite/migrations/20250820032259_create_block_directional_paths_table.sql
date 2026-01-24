-- Migration: create_block_directional_paths
-- Create table to store all paths between blocks in the directional hierarchy
CREATE TABLE IF NOT EXISTS block_directional_paths (
      id TEXT PRIMARY KEY NOT NULL,
      block_ancestor_id TEXT NOT NULL,
      block_descendant_id TEXT NOT NULL,
      block_path_ids TEXT NOT NULL, -- JSON array of block UUIDs representing the path
      path_length INTEGER NOT NULL,
      created_at TEXT NOT NULL,
      FOREIGN KEY (block_ancestor_id) REFERENCES blocks (id) ON DELETE CASCADE,
      FOREIGN KEY (block_descendant_id) REFERENCES blocks (id) ON DELETE CASCADE,
      UNIQUE(block_path_ids) -- Path itself is unique
  );

-- Create indexes for efficient querying
CREATE INDEX idx_block_directional_paths_ancestor ON block_directional_paths (block_ancestor_id);
CREATE INDEX idx_block_directional_paths_descendant ON block_directional_paths (block_descendant_id);
CREATE INDEX idx_block_directional_paths_length ON block_directional_paths (path_length);

-- Composite indexes for common tree queries
CREATE INDEX idx_block_directional_paths_ancestor_length ON block_directional_paths (block_ancestor_id, path_length);
CREATE INDEX idx_block_directional_paths_descendant_length ON block_directional_paths (block_descendant_id, path_length);