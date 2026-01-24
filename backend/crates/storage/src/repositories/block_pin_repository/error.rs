use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum BlockPinError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Block pin record not found: {block_id}")]
    NotFound { block_id: Uuid },

    #[error("Block not found: {block_id}")]
    BlockNotFound { block_id: Uuid },
}

pub type BlockPinResult<T> = Result<T, BlockPinError>;
