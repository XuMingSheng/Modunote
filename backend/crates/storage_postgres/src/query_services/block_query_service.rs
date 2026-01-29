use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Executor, Postgres};
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
pub struct PostgresBlockQueryService;

impl PostgresBlockQueryService {
    pub fn new() -> Self {
        Default::default()
    }
}

#[async_trait]
impl BlockQueryService<Postgres> for PostgresBlockQueryService {
    async fn get_opened<'e, E>(&self, executor: E) -> Result<Vec<OpenedBlockDto>>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let opened_blocks = sqlx::query_as!(
            OpenedBlockModel,
            r#"
            SELECT 
                b.id, 
                b.title, 
                bo.opened_at,
                bo.tab_index
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
        E: Executor<'e, Database = Postgres>,
    {
        let sql_query_str = format!("%{query}%");

        let blocks = sqlx::query_as!(
            BlockSummaryDto,
            r#"
            SELECT
                b.id, 
                b.title,
                b.created_at,
                b.updated_at,
                bo.opened_at as "opened_at?"
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
