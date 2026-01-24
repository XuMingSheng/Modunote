use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum CanvasPinError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Canvas pin record not found: {canvas_id}")]
    NotFound { canvas_id: Uuid },

    #[error("Canvas not found: {canvas_id}")]
    CanvasNotFound { canvas_id: Uuid },
}

pub type CanvasPinResult<T> = Result<T, CanvasPinError>;
