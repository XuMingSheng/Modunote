#[derive(thiserror::Error, Debug)]
pub enum BlockLinkQueryServiceError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

pub type BlockLinkQueryServiceResult<T> = Result<T, BlockLinkQueryServiceError>;
