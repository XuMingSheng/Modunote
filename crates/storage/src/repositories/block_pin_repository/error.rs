use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum BlockPinError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Block pin record not found: {id}")]
    NotFound { id: Uuid },
}

pub type BlockPinResult<T> = Result<T, BlockPinError>;