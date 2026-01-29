use async_trait::async_trait;
use sqlx::{Acquire, Database, Executor};
use uuid::Uuid;

use super::dtos::CreateBlockRelatedLinkDto;
use super::error::BlockRelatedLinkResult as Result;
use domain::blocks::BlockRelatedLink;

#[async_trait]
pub trait BlockRelatedLinkRepository<DB: Database>: Send + Sync {
    async fn get_by_id<'e, E>(&self, id: Uuid, executor: E) -> Result<Option<BlockRelatedLink>>
    where
        E: Executor<'e, Database = DB>;

    async fn create<'e, E>(
        &self,
        input: &CreateBlockRelatedLinkDto,
        executor: E,
    ) -> Result<BlockRelatedLink>
    where
        E: Executor<'e, Database = DB> + Acquire<'e, Database = DB>;

    async fn delete_by_id<'e, E>(&self, id: Uuid, executor: E) -> Result<()>
    where
        E: Executor<'e, Database = DB> + Acquire<'e, Database = DB>;

    async fn delete_by_block_ids<'e, E>(
        &self,
        block_a_id: Uuid,
        block_b_id: Uuid,
        executor: E,
    ) -> Result<()>
    where
        E: Executor<'e, Database = DB> + Acquire<'e, Database = DB>;
}
