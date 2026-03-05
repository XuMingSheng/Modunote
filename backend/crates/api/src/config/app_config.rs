use serde::Deserialize;
use std::path::PathBuf;

use super::{error::ConfigError, error::ConfigResult as Result, utils::load_value};
use crate::telemetry::TelemetryConfig;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub frontend_url: String,
    pub telemetry: TelemetryConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        if let Err(e) = dotenvy::dotenv() {
            eprintln!("Warning: Failed to load .env file: {e}");
        }

        let env = std::env::var("APP_ENV")?;
        let database_url = resolve_database_url(&env)?;
        let table = load_toml(&env)?;

        let config = Self {
            database_url,
            frontend_url: load_value("FRONTEND_URL", "frontend_url", &table)?,
            telemetry: TelemetryConfig::load(&table)?,
        };

        Ok(config)
    }
}

fn load_toml(env: &str) -> Result<toml::Table> {
    let path = PathBuf::from("configs").join(format!("config.{env}.toml"));

    let content = std::fs::read_to_string(&path).map_err(|e| ConfigError::FileRead {
        path: path.clone(),
        source: e,
    })?;

    let table: toml::Table = toml::from_str(&content)?;

    Ok(table)
}

fn resolve_database_url(env: &str) -> Result<String> {
    if env == "dev" {
        let url = std::env::var("DATABASE_URL")?;
        Ok(url)
    } else if env == "cloud" {
        let host = std::env::var("DB_HOST")?;
        let user = std::env::var("DB_USER")?;
        let pass = std::env::var("DB_PASSWORD")?;
        let db_name = std::env::var("DB_NAME")?;
        Ok(format!("postgresql://{user}:{pass}@{host}:5432/{db_name}"))
    } else {
        Err(ConfigError::MissingValue("DATABASE_URL".into()))
    }
}
