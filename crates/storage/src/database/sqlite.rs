use sqlx::{Pool, Sqlite};

use crate::database::error::DatabaseResult;

#[derive(Clone)]
pub struct SqliteDb {
    pool: Pool<Sqlite>,
}

impl SqliteDb {
    pub async fn new(database_url: &str) -> DatabaseResult<Self> {
        Self::create_directory_if_not_exist(database_url)?;

        let pool = sqlx::SqlitePool::connect(database_url).await?;

        let db = SqliteDb { pool };

        db.run_migration().await?;

        Ok(db)
    }

    pub fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }

    pub async fn run_migration(&self) -> DatabaseResult<()> {
        sqlx::migrate!("./migrations/sqlite")
            .run(&self.pool)
            .await?;
        Ok(())
    }

    fn create_directory_if_not_exist(database_url: &str) -> DatabaseResult<()> {
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
