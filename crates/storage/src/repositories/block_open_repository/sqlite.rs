use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use super::{
    error::BlockOpenResult as Result,
    traits::BlockOpenRepositoryTrait,
};
use crate::{BlockOpen, BlockOpenSummary};

#[derive(Clone, Debug)]
pub struct SqliteBlockOpenRepository {
    pool: Pool<Sqlite>,
}

impl SqliteBlockOpenRepository {
    pub fn new(pool: &Pool<Sqlite>) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait]
impl BlockOpenRepositoryTrait for SqliteBlockOpenRepository {
    async fn open(&self, block_id: Uuid) -> Result<BlockOpen> {
        self.compact_tab_orders().await?;

        let now = Utc::now();
        let is_opened = self.is_opened(block_id).await?;

        if is_opened {
            return self.update_opened_at(block_id, now).await;
        }

        let tab_order =
            sqlx::query_scalar!("SELECT COALESCE(MAX(tab_order), 0) + 1 FROM block_opens")
                .fetch_one(&self.pool)
                .await?;

        sqlx::query!(
            "INSERT INTO block_opens 
                (block_id, opened_at, tab_order)
                VALUES ($1, $2, $3)",
            block_id,
            now,
            tab_order
        )
        .execute(&self.pool)
        .await?;

        self.get_by_id(block_id).await
    }

    async fn close(&self, block_id: Uuid) -> Result<()> {
        sqlx::query!("DELETE FROM block_opens WHERE block_id = $1", block_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn is_opened(&self, block_id: Uuid) -> Result<bool> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM block_opens WHERE block_id = $1",
            block_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count > 0)
    }

    async fn get_opened_summary(&self) -> Result<Vec<BlockOpenSummary>> {
        let opened_blocks = sqlx::query_as!(
            BlockOpenSummary,
            r#"SELECT 
                bo.block_id as "block_id: _", 
                b.title, 
                bo.opened_at as "opened_at: _",
                bo.tab_order,
                bp.pinned_at as "pinned_at: _"
            FROM block_opens bo
            JOIN blocks b on bo.block_id = b.id
            LEFT JOIN block_pins bp on bo.block_id = bp.block_id
            ORDER BY bo.tab_order ASC"#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(opened_blocks)
    }

    async fn close_all(&self) -> Result<()> {
        sqlx::query!("DELETE FROM block_opens")
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

impl SqliteBlockOpenRepository {
    async fn get_by_id(&self, block_id: Uuid) -> Result<BlockOpen> {
        let block_open = sqlx::query_as!(
            BlockOpen,
            r#"
            SELECT 
                block_id as "block_id: _", 
                opened_at as "opened_at: _",
                tab_order
            FROM block_opens
            WHERE block_id = $1
            "#,
            block_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(block_open)
    }

    async fn compact_tab_orders(&self) -> Result<()> {
        sqlx::query!(
            "UPDATE block_opens SET tab_order = (
                SELECT COUNT(*) FROM block_opens bo2
                WHERE bo2.tab_order <= block_opens.tab_order
            )"
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_opened_at(
        &self,
        block_id: Uuid,
        timestamp: DateTime<Utc>,
    ) -> Result<BlockOpen> {
        sqlx::query!(
            "UPDATE block_opens SET opened_at = $1 WHERE block_id = $2",
            timestamp,
            block_id
        )
        .execute(&self.pool)
        .await?;

        self.get_by_id(block_id).await
    }
}