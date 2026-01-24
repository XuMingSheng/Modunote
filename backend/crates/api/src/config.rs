use std::env;

cfg_if::cfg_if! {
    if #[cfg(feature = "native")] {
        pub use storage_sqlite::SqliteDb as DatabaseImpl;

        pub use storage_sqlite::repositories::{
            SqliteBlockRepository as BlockRepositoryImpl,
            SqliteBlockDirectionalLinkRepository as BlockDirectionalLinkRepositoryImpl,
            SqliteBlockRelatedLinkRepository as BlockRelatedLinRepositoryImpl,
            SqliteWorkspaceRepository as WorkspaceRepositoryImpl
        };

        pub use storage_sqlite::query_services::{
            SqliteBlockQueryService as BlockQueryServiceImpl,
            SqliteBlockLinkQueryService as BlockLinkQueryServiceImpl,
        };
    }
}

use super::AppResult as Result;

pub struct AppConfig {
    pub database_url: String,
    pub log_dir_path: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        dotenvy::from_path("backend/.env").ok();
        let database_url = env::var("DATABASE_URL")?;
        let log_dir_path: String = env::var("LOG_DIR_PATH")?;

        let config = Self {
            database_url,
            log_dir_path,
        };

        Ok(config)
    }
}
