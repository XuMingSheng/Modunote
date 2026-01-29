use tracing::debug;

use super::AppResult as Result;

pub struct AppConfig {
    pub database_url: String,
    pub log_dir_path: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        dotenvy::from_path("backend/.env").unwrap_or_else(|e| debug!("Error loading .env: {e}"));

        let database_url = std::env::var("DATABASE_URL")?;
        let log_dir_path: String = std::env::var("LOG_DIR_PATH")?;

        let config = Self {
            database_url,
            log_dir_path,
        };

        Ok(config)
    }
}
