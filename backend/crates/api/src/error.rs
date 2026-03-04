use std::path::PathBuf;
use thiserror::Error;

use crate::telemetry::TelemetryError;
use storage::database::DatabaseError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Environment variable error: {0}")]
    Environment(#[from] std::env::VarError),

    #[error("Dotenv error: {0}")]
    DotEnv(#[from] dotenvy::Error),

    #[error("Toml deserializing error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("Database error: {0}")]
    Database(#[from] DatabaseError),

    #[error("Telemetry error: {0}")]
    Telemetry(#[from] TelemetryError),

    #[error("Failed to read config file at {path}: {source}")]
    ConfigRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Configuration error: {0}")]
    MissingConfig(String),

    #[error("Parse error for {var_name}: {source}")]
    ParseError {
        var_name: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

pub type AppResult<T> = std::result::Result<T, AppError>;
