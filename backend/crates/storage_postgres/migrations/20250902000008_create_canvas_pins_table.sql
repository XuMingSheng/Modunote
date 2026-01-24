CREATE TABLE IF NOT EXISTS canvas_pins (
    canvas_id UUID PRIMARY KEY NOT NULL,
    pinned_at TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (canvas_id) REFERENCES canvases (id) ON DELETE CASCADE
);
