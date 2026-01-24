use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use super::{error::BlockPinResult as Result, traits::BlockPinRepositoryTrait};
use crate::{
    entities::{BlockPin, BlockSummary},
    helpers::sqlx_error_kind_helpers::is_foreign_key_violation,
    repositories::block_pin_repository::BlockPinError,
};

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
    async fn is_pinned(&self, block_id: Uuid) -> Result<bool> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS (SELECT 1 FROM block_pins WHERE block_id = $1)",
            block_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(exists > 0)
    }

    async fn pin(&self, block_id: Uuid) -> Result<BlockPin> {
        let now = Utc::now();

        let block_pin = sqlx::query_as!(
            BlockPin,
            r#"
            INSERT OR REPLACE INTO block_pins
                (block_id, pinned_at)
                VALUES ($1, $2)
            RETURNING
                block_id as "block_id: _",
                pinned_at as "pinned_at: _"
            "#,
            block_id,
            now,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if is_foreign_key_violation(&e) {
                return BlockPinError::BlockNotFound { block_id };
            }
            BlockPinError::Database(e)
        })?;

        Ok(block_pin)
    }

    async fn unpin(&self, block_id: Uuid) -> Result<()> {
        let result = sqlx::query!("DELETE FROM block_pins WHERE block_id = $1", block_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(BlockPinError::NotFound { block_id });
        }

        Ok(())
    }

    async fn get_pinned_blocks(&self) -> Result<Vec<BlockSummary>> {
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
