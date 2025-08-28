use uuid::Uuid;

use crate::helpers::BlockDirectionalPathHelperError;

#[derive(thiserror::Error, Debug)]
pub enum BlockError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Block not found: {id}")]
    NotFound { id: Uuid },

    #[error("Path helper error: {0}")]
    PathHelper(#[from] BlockDirectionalPathHelperError),
}

pub type BlockResult<T> = Result<T, BlockError>;
