CREATE TABLE canvas_pins (
    canvas_id BLOB PRIMARY KEY NOT NULL,
    pinned_at TEXT NOT NULL,
    FOREIGN KEY (canvas_id) REFERENCES canvases (id) ON DELETE CASCADE
);