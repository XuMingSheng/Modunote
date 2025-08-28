use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum BlockDirectionalPathHelperError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Empty path provided - cannot create path with no blocks")]
    EmptyPath,

    #[error("Self path detected - cannot create path from block {id} to itself")]
    SelfPath { id: Uuid },
}

pub type BlockDirectionalPathHelperResult<T> = Result<T, BlockDirectionalPathHelperError>;
