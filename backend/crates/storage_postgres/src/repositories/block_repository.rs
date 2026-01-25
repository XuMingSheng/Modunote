use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Acquire, Executor, Postgres};
use uuid::Uuid;

use crate::helpers::PostgresBlockDirectionalPathHelper;
use domain::blocks::Block;
use storage::helpers::block_directional_path_helper::BlockDirectionalPathHelper;
use storage::repositories::block_repository::{
    BlockRepository, BlockRepositoryError, BlockRepostoryResult as Result,
};

#[derive(Clone, Debug, Default)]
pub struct PostgresBlockRepository {
    path_helper: PostgresBlockDirectionalPathHelper,
}

impl PostgresBlockRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl BlockRepository<Postgres> for PostgresBlockRepository {
    async fn get_by_id<'e, E>(&self, id: Uuid, executor: E) -> Result<Option<Block>>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let block = sqlx::query_as!(
            Block,
            r#"SELECT
                id,
                title,
                content,
                created_at,
                updated_at
            FROM blocks
            WHERE id = $1"#,
            id
        )
        .fetch_optional(executor)
        .await?;

        Ok(block)
    }

    async fn delete_by_id<'e, E>(&self, id: Uuid, executor: E) -> Result<()>
    where
        E: Executor<'e, Database = Postgres> + Acquire<'e, Database = Postgres>,
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
        E: Executor<'e, Database = Postgres>,
    {
        let now = Utc::now();

        sqlx::query!(
            r#"
            INSERT INTO blocks
                (id, title, content, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                title = EXCLUDED.title,
                content = EXCLUDED.content,
                created_at = EXCLUDED.created_at,
                updated_at = EXCLUDED.updated_at
            "#,
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
