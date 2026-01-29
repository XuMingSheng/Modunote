use uuid::Uuid;

use crate::helpers::block_directional_path_helper::BlockDirectionalPathHelperError;

#[derive(thiserror::Error, Debug)]
pub enum BlockRepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Path helper error: {0}")]
    PathHelper(#[from] BlockDirectionalPathHelperError),

    #[error("Block not found: {id}")]
    NotFound { id: Uuid },
}

pub type BlockRepostoryResult<T> = Result<T, BlockRepositoryError>;
