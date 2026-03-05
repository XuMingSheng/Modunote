use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Environment variable error: {0}")]
    Environment(#[from] std::env::VarError),

    #[error("Dotenv error: {0}")]
    DotEnv(#[from] dotenvy::Error),

    #[error("Toml deserializing error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("Missing value from configuration: {0}")]
    MissingValue(String),

    #[error("Missing section from configuration: {0}")]
    MissingSection(String),

    #[error("Failed to read config file at {path}: {source}")]
    FileRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Parse error for {var_name}: {source}")]
    ParseError {
        var_name: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

pub type ConfigResult<T> = std::result::Result<T, ConfigError>;
