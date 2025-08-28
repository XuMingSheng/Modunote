use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum BlockRelatedLinkError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Cannot link block to itself: {id}")]
    SelfLink { id: Uuid },

    #[error("Link already exists: {a} <-> {b}")]
    AlreadyExists { a: Uuid, b: Uuid },

    #[error("Link not found: {a} <-> {b}")]
    NotFound { a: Uuid, b: Uuid },
}

pub type BlockRelatedLinkResult<T> = Result<T, BlockRelatedLinkError>;