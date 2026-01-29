-- Migration: create_canvas_tables
-- Create tables for canvas functionality

-- Canvas table
CREATE TABLE IF NOT EXISTS canvases (
    id BLOB PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    viewport_x REAL NOT NULL DEFAULT 0.0,
    viewport_y REAL NOT NULL DEFAULT 0.0,
    zoom_level REAL NOT NULL DEFAULT 1.0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Canvas blocks positioning table
CREATE TABLE IF NOT EXISTS canvas_blocks (
    id BLOB PRIMARY KEY NOT NULL,
    canvas_id BLOB NOT NULL,
    block_id BLOB NOT NULL,
    x REAL NOT NULL,
    y REAL NOT NULL,
    z INTEGER NOT NULL DEFAULT 0,
    width REAL NOT NULL,
    height REAL NOT NULL,
    scale REAL NOT NULL DEFAULT 1.0,
    content_visible BOOLEAN NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (canvas_id) REFERENCES canvases (id) ON DELETE CASCADE,
    FOREIGN KEY (block_id) REFERENCES blocks (id) ON DELETE CASCADE,
    UNIQUE(canvas_id, block_id)
);

-- Create indexes for efficient querying
CREATE INDEX idx_canvas_blocks_canvas_id ON canvas_blocks (canvas_id);
CREATE INDEX idx_canvas_blocks_block_id ON canvas_blocks (block_id);