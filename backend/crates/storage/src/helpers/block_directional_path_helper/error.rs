use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum BlockDirectionalPathHelperError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Empty path provided - cannot create path with no blocks")]
    EmptyPathCreation,

    #[error("Cyclic path provided - cannot create")]
    CyclicPathCreation,

    #[error("Path already exists: {from} -> {to}")]
    AlreadyExists { from: Uuid, to: Uuid },

    #[error("Block not found: {from} or {to}")]
    PathBlocksNotFound { from: Uuid, to: Uuid },

    #[error("Block not found: {block_id}")]
    BlockNotFound { block_id: Uuid },
}

pub type BlockDirectionalPathHelperResult<T> = Result<T, BlockDirectionalPathHelperError>;
