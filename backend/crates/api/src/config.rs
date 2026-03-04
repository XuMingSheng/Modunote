use serde::Deserialize;
use std::path::PathBuf;

use crate::{AppError, AppResult as Result, telemetry::TelemetryConfig};

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub telemetry: TelemetryConfig,
    pub database_url: String,
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
            telemetry: TelemetryConfig::load(&table)?,
            database_url,
        };

        Ok(config)
    }
}

fn load_toml(env: &str) -> Result<toml::Table> {
    let path = PathBuf::from("configs").join(format!("config.{env}.toml"));

    let content = std::fs::read_to_string(&path).map_err(|e| AppError::ConfigRead {
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
        Err(AppError::MissingConfig("DATABASE_URL".into()))
    }
}
