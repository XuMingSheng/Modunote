use crate::Result;
use sqlx::{Pool, Postgres};
use std::env;

#[derive(Clone)]
pub struct PostgresDb {
    pub pool: Pool<Postgres>,
}

impl PostgresDb {
    pub async fn new(database_url: &str) -> Result<Self> {
        Self::create_directory_if_not_exist(database_url)?;

        let pool = sqlx::PgPool::connect(database_url).await?;

        sqlx::migrate!("./migrations/postgres").run(&pool).await?;

        Ok(PostgresDb { pool })
    }

    pub fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}
