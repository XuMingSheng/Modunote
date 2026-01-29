use uuid::Uuid;

use crate::helpers::block_directional_path_helper::BlockDirectionalPathHelperError;

#[derive(thiserror::Error, Debug)]
pub enum BlockDirectionalLinkRepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Path helper error: {0}")]
    PathHelper(#[from] BlockDirectionalPathHelperError),

    #[error("Cycle would be created: {from} -> {to}")]
    CycleDetected { from: Uuid, to: Uuid },

    #[error("Link not found by id: {id}")]
    NotFoundById { id: Uuid },

    #[error("Link not found: {from} -> {to}")]
    NotFoundByBlocks { from: Uuid, to: Uuid },

    #[error("Link already exists: {from} -> {to}")]
    AlreadyExists { from: Uuid, to: Uuid },

    #[error("Blocks not found: {from} or {to}")]
    BlocksNotFound { from: Uuid, to: Uuid },
}

pub type BlockDirectionalLinkRepositoryResult<T> = Result<T, BlockDirectionalLinkRepositoryError>;
