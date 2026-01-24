use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use storage::database::{Database, DatabaseResult as Result};

#[derive(Clone, Debug)]
pub struct PostgresDb {
    pool: Pool<Postgres>,
}

#[async_trait]
impl Database for PostgresDb {
    type Provider = Postgres;

    fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }

    async fn connect(database_url: &str) -> Result<Self> {
        let pool = sqlx::PgPool::connect(database_url).await?;
        let db = PostgresDb { pool };
        Ok(db)
    }

    async fn run_migration(&self) -> Result<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }
}
