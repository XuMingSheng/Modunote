use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use crate::helpers::BlockDirectionalPathHelper;
use domain::blocks::Block;
use storage::repositories::BlockRepository;
use storage::repositories::block_repository::{
    BlockRepositoryError, BlockRepostoryResult as Result, CreateBlockDto, UpdateBlockDto,
};

#[derive(Clone, Debug)]
pub struct SqliteBlockRepository {
    pool: Pool<Sqlite>,
    path_helper: BlockDirectionalPathHelper,
}

impl SqliteBlockRepository {
    pub fn new(pool: &Pool<Sqlite>) -> Self {
        Self {
            pool: pool.clone(),
            path_helper: BlockDirectionalPathHelper,
        }
    }
}

#[async_trait]
impl BlockRepository for SqliteBlockRepository {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Block>> {
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
        .fetch_optional(&self.pool)
        .await?;

        Ok(block)
    }

    async fn create(&self, input: &CreateBlockDto) -> Result<Block> {
        let now = Utc::now();

        let block = sqlx::query_as!(
            Block,
            r#"
            INSERT INTO blocks 
                (id, title, content, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $4)
            RETURNING 
                id as "id: _",
                title,
                content,
                created_at "created_at: _",
                updated_at "updated_at: _"
            "#,
            input.id,
            input.title,
            input.content,
            now,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(block)
    }

    async fn update_by_id(&self, id: Uuid, input: &UpdateBlockDto) -> Result<Block> {
        let now = Utc::now();

        let block = sqlx::query_as!(
            Block,
            r#"
            UPDATE blocks SET
                title = COALESCE($2, title),
                content = COALESCE($3, content),
                updated_at = $4
            WHERE id = $1
            RETURNING 
                id as "id: _",
                title,
                content,
                created_at "created_at: _",
                updated_at "updated_at: _"
            "#,
            id,
            input.title,
            input.content,
            now
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| BlockRepositoryError::NotFound { id })?;

        Ok(block)
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<()> {
        let mut transaction = self.pool.begin().await?;

        self.path_helper
            .delete_paths_using_block(id, &mut transaction)
            .await?;

        let result = sqlx::query!("DELETE FROM blocks WHERE id = $1", id)
            .execute(&mut *transaction)
            .await?;

        if result.rows_affected() == 0 {
            return Err(BlockRepositoryError::NotFound { id });
        }

        transaction.commit().await?;

        Ok(())
    }

    async fn save(&self, block: &Block) -> Result<()> {
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
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
