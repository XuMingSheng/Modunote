-- Migration: convert_canvas_coordinates_to_grid
-- Convert canvas block coordinates from REAL to INTEGER for grid-based positioning

-- Rename existing table
ALTER TABLE canvas_blocks RENAME TO canvas_blocks_old;

-- Create new table with INTEGER grid coordinates
CREATE TABLE canvas_blocks (
    id BLOB PRIMARY KEY NOT NULL,
    canvas_id BLOB NOT NULL,
    block_id BLOB NOT NULL,
    grid_x INTEGER NOT NULL,
    grid_y INTEGER NOT NULL,
    grid_width INTEGER NOT NULL,
    grid_height INTEGER NOT NULL,
    z INTEGER NOT NULL DEFAULT 0,
    scale REAL NOT NULL DEFAULT 1.0,
    content_visible BOOLEAN NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (canvas_id) REFERENCES canvases (id) ON DELETE CASCADE,
    FOREIGN KEY (block_id) REFERENCES blocks (id) ON DELETE CASCADE,
    UNIQUE(canvas_id, block_id)
);

-- Copy data, converting REAL coordinates to INTEGER grid positions
INSERT INTO canvas_blocks (id, canvas_id, block_id, grid_x, grid_y, grid_width, grid_height, z, scale, content_visible, created_at, updated_at)
SELECT id, canvas_id, block_id,
        CAST(x AS INTEGER),
        CAST(y AS INTEGER),
        CAST(width AS INTEGER),
        CAST(height AS INTEGER),
        z, scale, content_visible, created_at, updated_at
FROM canvas_blocks_old;

-- Drop old table
DROP TABLE canvas_blocks_old;

-- Recreate indexes
CREATE INDEX idx_canvas_blocks_canvas_id ON canvas_blocks (canvas_id);
CREATE INDEX idx_canvas_blocks_block_id ON canvas_blocks (block_id);
