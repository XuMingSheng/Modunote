use async_trait::async_trait;
use sqlx::{Database, Executor};

use super::{
    dtos::{BlockSummaryDto, OpenedBlockDto},
    error::BlockQueryServiceResult as Result,
};

#[async_trait]
pub trait BlockQueryService<DB: Database>: Send + Sync {
    async fn get_opened<'e, E>(&self, executor: E) -> Result<Vec<OpenedBlockDto>>
    where
        E: Executor<'e, Database = DB>;

    async fn search<'e, E>(&self, query: &str, executor: E) -> Result<Vec<BlockSummaryDto>>
    where
        E: Executor<'e, Database = DB>;
}
