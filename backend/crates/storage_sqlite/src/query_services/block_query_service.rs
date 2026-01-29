use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Executor, Sqlite};
use uuid::Uuid;

use storage::query_services::BlockQueryService;
use storage::query_services::block_query_service::{
    BlockQueryServiceResult as Result, BlockSummaryDto, OpenedBlockDto,
};

struct OpenedBlockModel {
    id: Uuid,
    title: String,
    opened_at: DateTime<Utc>,
    tab_index: i32,
}

impl From<OpenedBlockModel> for OpenedBlockDto {
    fn from(model: OpenedBlockModel) -> Self {
        Self {
            id: model.id,
            title: model.title,
            opened_at: model.opened_at,
            tab_index: model.tab_index as usize,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct SqliteBlockQueryService;

impl SqliteBlockQueryService {
    pub fn new() -> Self {
        Default::default()
    }
}

#[async_trait]
impl BlockQueryService<Sqlite> for SqliteBlockQueryService {
    async fn get_opened<'e, E>(&self, executor: E) -> Result<Vec<OpenedBlockDto>>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let opened_blocks = sqlx::query_as!(
            OpenedBlockModel,
            r#"
            SELECT 
                b.id as "id: _", 
                b.title, 
                bo.opened_at as "opened_at: _",
                bo.tab_index as "tab_index: _"
            FROM block_opens bo
            JOIN blocks b on b.id = bo.block_id
            ORDER BY bo.tab_index ASC
            "#,
        )
        .fetch_all(executor)
        .await?
        .into_iter()
        .map(OpenedBlockDto::from)
        .collect();

        Ok(opened_blocks)
    }

    async fn search<'e, E>(&self, query: &str, executor: E) -> Result<Vec<BlockSummaryDto>>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let sql_query_str = format!("%{query}%");

        let blocks = sqlx::query_as!(
            BlockSummaryDto,
            r#"
            SELECT
                b.id as "id: _", 
                b.title,
                b.created_at as "created_at: _",
                b.updated_at as "updated_at: _",
                bo.opened_at as "opened_at: _"
            FROM blocks b
            LEFT JOIN block_opens bo ON bo.block_id = b.id
            WHERE 
                title LIKE $1 OR
                content LIKE $1
            ORDER BY updated_at DESC
            LIMIT 50
            "#,
            sql_query_str,
        )
        .fetch_all(executor)
        .await?;

        Ok(blocks)
    }
}
