use thiserror::Error;

use crate::{config::ConfigError, telemetry::TelemetryError};
use storage::database::DatabaseError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    ConfigError(#[from] ConfigError),

    #[error("Telemetry error: {0}")]
    Telemetry(#[from] TelemetryError),

    #[error("Database error: {0}")]
    Database(#[from] DatabaseError),

    #[error("Parse error for {var_name}: {source}")]
    ParseError {
        var_name: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

pub type AppResult<T> = std::result::Result<T, AppError>;
