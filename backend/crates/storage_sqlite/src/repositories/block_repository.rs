use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Acquire, Executor, Sqlite};
use uuid::Uuid;

use crate::helpers::SqliteBlockDirectionalPathHelper;
use domain::blocks::Block;
use storage::helpers::block_directional_path_helper::BlockDirectionalPathHelper;
use storage::repositories::BlockRepository;
use storage::repositories::block_repository::{
    BlockRepositoryError, BlockRepostoryResult as Result,
};

#[derive(Clone, Debug, Default)]
pub struct SqliteBlockRepository {
    path_helper: SqliteBlockDirectionalPathHelper,
}

impl SqliteBlockRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl BlockRepository<Sqlite> for SqliteBlockRepository {
    async fn get_by_id<'e, E>(&self, id: Uuid, executor: E) -> Result<Option<Block>>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let block = sqlx::query_as!(
            Block,
            r#"SELECT 
                id as "id: _" , 
                title, 
                content, 
                created_at as "created_at: _", 
                updated_at as "updated_at: _" 
            FROM blocks 
            WHERE id = $1"#,
            id,
        )
        .fetch_optional(executor)
        .await?;

        Ok(block)
    }

    async fn delete_by_id<'e, E>(&self, id: Uuid, executor: E) -> Result<()>
    where
        E: Executor<'e, Database = Sqlite> + Acquire<'e, Database = Sqlite>,
    {
        let mut conn = executor.acquire().await?;
        let mut tx = conn.begin().await?;

        self.path_helper
            .delete_paths_using_block(id, &mut *tx)
            .await?;

        let result = sqlx::query!("DELETE FROM blocks WHERE id = $1", id)
            .execute(&mut *tx)
            .await?;

        if result.rows_affected() == 0 {
            return Err(BlockRepositoryError::NotFound { id });
        }

        tx.commit().await?;

        Ok(())
    }

    async fn save<'e, E>(&self, block: &Block, executor: E) -> Result<()>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let now = Utc::now();

        sqlx::query!(
            "INSERT OR REPLACE INTO blocks 
                (id, title, content, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5)",
            block.id,
            block.title,
            block.content,
            block.created_at,
            now,
        )
        .execute(executor)
        .await?;

        Ok(())
    }
}
