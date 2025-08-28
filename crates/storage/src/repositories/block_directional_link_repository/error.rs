use uuid::Uuid;

use crate::helpers::BlockDirectionalPathHelperError;

#[derive(thiserror::Error, Debug)]
pub enum BlockDirectionalLinkError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Cycle would be created: {from} -> {to}")]
    CycleDetected { from: Uuid, to: Uuid },

    #[error("Cannot link block to itself: {id}")]
    SelfLink { id: Uuid },

    #[error("Link already exists: {from} -> {to}")]
    AlreadyExists { from: Uuid, to: Uuid },

    #[error("Link not found: {from} -> {to}")]
    NotFound { from: Uuid, to: Uuid },

    #[error("Path helper error: {0}")]
    PathHelper(#[from] BlockDirectionalPathHelperError),
}

pub type BlockDirectionalLinkResult<T> = Result<T, BlockDirectionalLinkError>;
