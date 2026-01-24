use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum CanvasBlockError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Block placment in canvas not found: {id}")]
    NotFound { id: Uuid },

    #[error("Canvas not found {canvas_id}")]
    CanvasNotFound { canvas_id: Uuid },

    #[error("Block not found {block_id}")]
    BlockNotFound { block_id: Uuid },
}

pub type CanvasBlockResult<T> = Result<T, CanvasBlockError>;
