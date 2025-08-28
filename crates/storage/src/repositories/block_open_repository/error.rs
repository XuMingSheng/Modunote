use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum BlockOpenError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Block open record not found: {id}")]
    NotFound { id: Uuid },
}

pub type BlockOpenResult<T> = Result<T, BlockOpenError>;