use thiserror::Error;

#[derive(Error, Debug)]
pub enum BackendError {
    #[error("Database error: {0}")]
    Database(#[from] storage::DatabaseError),

    #[error("Environment variable error: {0}")]
    Environment(#[from] std::env::VarError),

    #[error("Dotenv error: {0}")]
    DotEnv(#[from] dotenvy::Error),

    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, BackendError>;
