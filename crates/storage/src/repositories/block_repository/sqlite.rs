use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Pool, Sqlite, query_builder::QueryBuilder};
use uuid::Uuid;

use super::{
    error::{BlockError, BlockResult as Result},
    traits::BlockRepositoryTrait,
};
use crate::{
    Block, BlockCreate, BlockSummary, BlockUpdate,
    helpers::{BlockDirectionalPathHelper, BlockDirectionalPathHelperTrait},
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
            path_helper: BlockDirectionalPathHelper::new(pool),
        }
    }
}

#[async_trait]
impl BlockRepositoryTrait for SqliteBlockRepository {
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

    async fn create(&self, block_create: BlockCreate) -> Result<Block> {
        let now = Utc::now();

        sqlx::query!(
            "INSERT INTO blocks (id,
            title, content, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $4)",
            block_create.id,
            block_create.title,
            block_create.content,
            now,
        )
        .execute(&self.pool)
        .await?;

        self.get_by_id(block_create.id)
            .await?
            .ok_or(BlockError::NotFound {
                id: block_create.id,
            })
    }

    async fn update_by_id(&self, id: Uuid, block_update: &BlockUpdate) -> Result<Block> {
        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new("UPDATE blocks SET ");
        let mut has_query_fields = false;

        if let Some(title) = &block_update.title {
            query_builder.push("title = ").push_bind(title).push(", ");
            has_query_fields = true;
        }

        if let Some(content) = &block_update.content {
            query_builder
                .push("content = ")
                .push_bind(content)
                .push(", ");
            has_query_fields = true;
        }

        if has_query_fields {
            query_builder.push("updated_at = ").push_bind(Utc::now());
            query_builder.push(" WHERE id = ").push_bind(id);

            let query = query_builder.build();
            let result = query.execute(&self.pool).await?;

            if result.rows_affected() == 0 {
                return Err(BlockError::NotFound { id });
            }
        }

        self.get_by_id(id).await?.ok_or(BlockError::NotFound { id })
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<()> {
        let mut transaction = self.pool.begin().await?;

        self.path_helper
            .delete_path_using_block(id, &mut transaction)
            .await?;

        let result = sqlx::query!("DELETE FROM blocks WHERE id = $1", id)
            .execute(&mut *transaction)
            .await?;

        if result.rows_affected() == 0 {
            return Err(BlockError::NotFound { id });
        }

        transaction.commit().await?;

        Ok(())
    }

    async fn search(&self, query: &str) -> Result<Vec<BlockSummary>> {
        let sql_query_str = format!("%{query}%");

        let blocks = sqlx::query_as!(
            BlockSummary,
            r#"
            SELECT
                b.id as "id: _", 
                b.title,
                b.created_at as "created_at: _",
                b.updated_at as "updated_at: _",
                bo.opened_at as "opened_at: _",
                bp.pinned_at as "pinned_at: _"
            FROM blocks b
            LEFT JOIN block_opens bo ON bo.block_id = b.id
            LEFT JOIN block_pins bp ON bp.block_id = b.id
            WHERE 
                title LIKE $1 OR
                content LIKE $1
            ORDER BY updated_at DESC
            LIMIT 50
            "#,
            sql_query_str,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(blocks)
    }
}
