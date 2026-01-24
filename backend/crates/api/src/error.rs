use thiserror::Error;

use storage::database::DatabaseError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Environment variable error: {0}")]
    Environment(#[from] std::env::VarError),

    #[error("Dotenv error: {0}")]
    DotEnv(#[from] dotenvy::Error),

    #[error("Database error: {0}")]
    Database(#[from] DatabaseError),
}

pub type AppResult<T> = std::result::Result<T, AppError>;
