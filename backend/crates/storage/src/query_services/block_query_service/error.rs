#[derive(thiserror::Error, Debug)]
pub enum BlockQueryServiceError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

pub type BlockQueryServiceResult<T> = Result<T, BlockQueryServiceError>;
