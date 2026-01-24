use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum CanvasError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Canvas not found: {id}")]
    NotFound { id: Uuid },
}

pub type CanvasResult<T> = Result<T, CanvasError>;
