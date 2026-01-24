use async_trait::async_trait;
use sqlx::{Pool, Sqlite};

use storage::database::{Database, DatabaseResult as Result};

#[derive(Clone, Debug)]
pub struct SqliteDb {
    pool: Pool<Sqlite>,
}

#[async_trait]
impl Database for SqliteDb {
    type Provider = Sqlite;

    fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }

    async fn connect(database_url: &str) -> Result<Self> {
        Self::create_directory_if_not_exist(database_url)?;

        let pool = sqlx::SqlitePool::connect(database_url).await?;

        let db = SqliteDb { pool };

        Ok(db)
    }

    async fn run_migration(&self) -> Result<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }
}

impl SqliteDb {
    fn create_directory_if_not_exist(database_url: &str) -> Result<()> {
        // Assume absolute path for now
        if let Some(db_path_str) = database_url.strip_prefix("sqlite:") {
            let db_path = std::path::Path::new(db_path_str);

            if let Some(parent) = db_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
        }

        Ok(())
    }
}
