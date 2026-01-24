CREATE TABLE IF NOT EXISTS canvas_blocks (
    id UUID PRIMARY KEY NOT NULL,
    canvas_id UUID NOT NULL,
    block_id UUID NOT NULL,
    grid_x INTEGER NOT NULL,
    grid_y INTEGER NOT NULL,
    grid_width INTEGER NOT NULL,
    grid_height INTEGER NOT NULL,
    z INTEGER NOT NULL DEFAULT 0,
    scale DOUBLE PRECISION NOT NULL DEFAULT 1.0,
    content_visible BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (canvas_id) REFERENCES canvases (id) ON DELETE CASCADE,
    FOREIGN KEY (block_id) REFERENCES blocks (id) ON DELETE CASCADE,
    UNIQUE(canvas_id, block_id)
);

CREATE INDEX IF NOT EXISTS idx_canvas_blocks_canvas_id
    ON canvas_blocks (canvas_id);
CREATE INDEX IF NOT EXISTS idx_canvas_blocks_block_id
    ON canvas_blocks (block_id);
