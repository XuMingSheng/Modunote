use async_trait::async_trait;
use sqlx::{Database, Executor};
use uuid::Uuid;

use super::dtos::{AllLinkedBlocksDto, LinkedBlockDto};
use super::error::BlockLinkQueryServiceResult as Result;

#[async_trait]
pub trait BlockLinkQueryService<DB: Database>: Send + Sync {
    async fn get_linked_blocks<'e, E>(
        &self,
        block_id: Uuid,
        executor: E,
    ) -> Result<AllLinkedBlocksDto>
    where
        E: Executor<'e, Database = DB>;

    async fn get_parent_blocks<'e, E>(&self, block_id: Uuid, executor: E)
        -> Result<Vec<LinkedBlockDto>>
    where
        E: Executor<'e, Database = DB>;

    async fn get_child_blocks<'e, E>(&self, block_id: Uuid, executor: E)
        -> Result<Vec<LinkedBlockDto>>
    where
        E: Executor<'e, Database = DB>;

    async fn get_related_blocks<'e, E>(&self, block_id: Uuid, executor: E)
        -> Result<Vec<LinkedBlockDto>>
    where
        E: Executor<'e, Database = DB>;
}
