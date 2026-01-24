use async_trait::async_trait;
use sqlx::{Pool, Sqlite};

use storage::query_services::BlockQueryService;
use storage::query_services::block_query_service::{
    BlockQueryServiceResult as Result, BlockSummaryDto, OpenedBlockDto,
};

#[derive(Clone, Debug)]
pub struct SqliteBlockQueryService {
    pool: Pool<Sqlite>,
}

impl SqliteBlockQueryService {
    pub fn new(pool: &Pool<Sqlite>) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait]
impl BlockQueryService for SqliteBlockQueryService {
    async fn get_opened(&self) -> Result<Vec<OpenedBlockDto>> {
        let opened_blocks = sqlx::query_as!(
            OpenedBlockDto,
            r#"
            SELECT 
                b.id as "id: _", 
                b.title, 
                bo.opened_at as "opened_at: _",
                bo.tab_order as "tab_order: _"
            FROM block_opens bo
            JOIN blocks b on b.id = bo.block_id
            ORDER BY bo.tab_order ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(opened_blocks)
    }

    async fn search(&self, query: &str) -> Result<Vec<BlockSummaryDto>> {
        let sql_query_str = format!("%{query}%");

        let blocks = sqlx::query_as!(
            BlockSummaryDto,
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
