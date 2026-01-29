use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum BlockDagQueryServiceError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Cycle would be created: {from} -> {to}")]
    CycleDetected { from: Uuid, to: Uuid },

    #[error("Cannot link block to itself: {id}")]
    SelfLink { id: Uuid },

    #[error("Link not found: {from} -> {to}")]
    NotFound { from: Uuid, to: Uuid },

    #[error("Link already exists: {from} -> {to}")]
    AlreadyExists { from: Uuid, to: Uuid },

    #[error("Block not found: {from} or {to}")]
    LinkBlocksNotFound { from: Uuid, to: Uuid },

    #[error("Block not found: {block_id}")]
    BlockNotFound { block_id: Uuid },
}

pub type BlockDagQueryServiceResult<T> = Result<T, BlockDagQueryServiceError>;
