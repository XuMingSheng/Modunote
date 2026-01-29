#[derive(thiserror::Error, Debug)]
pub enum WorkspaceRepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Some blocks not found")]
    SomeBlocksNotFound,
}

pub type WorkspaceRepostoryResult<T> = Result<T, WorkspaceRepositoryError>;
