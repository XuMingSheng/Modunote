use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use super::{
    error::BlockPinResult as Result,
    traits::BlockPinRepositoryTrait,
};
use crate::BlockSummary;

#[derive(Clone, Debug)]
pub struct SqliteBlockPinRepository {
    pool: Pool<Sqlite>,
}

impl SqliteBlockPinRepository {
    pub fn new(pool: &Pool<Sqlite>) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait]
impl BlockPinRepositoryTrait for SqliteBlockPinRepository {
    async fn pin(&self, block_id: Uuid) -> Result<()> {
        let now = Utc::now();

        sqlx::query!(
            "INSERT OR REPLACE INTO block_pins
                (block_id, pinned_at)
                VALUES ($1, $2)",
            block_id,
            now,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn unpin(&self, block_id: Uuid) -> Result<()> {
        sqlx::query!("DELETE FROM block_pins WHERE block_id = $1", block_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn is_pinned(&self, block_id: Uuid) -> Result<bool> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM block_pins WHERE block_id = $1",
            block_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count > 0)
    }

    async fn get_pinned_summary(&self) -> Result<Vec<BlockSummary>> {
        let pinned_blocks = sqlx::query_as!(
            BlockSummary,
            r#"SELECT 
                b.id as "id: _", 
                b.title, 
                b.created_at as "created_at: _",
                b.updated_at as "updated_at: _",
                bo.opened_at as "opened_at: _",
                bp.pinned_at as "pinned_at: _"
            FROM block_pins bp
            JOIN blocks b on bp.block_id = b.id
            LEFT JOIN block_opens bo on bp.block_id = bo.block_id
            ORDER BY bp.pinned_at DESC"#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(pinned_blocks)
    }

    async fn unpin_all(&self) -> Result<()> {
        sqlx::query!("DELETE FROM block_pins")
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}