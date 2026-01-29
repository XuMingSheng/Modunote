use async_trait::async_trait;
use sqlx::{Acquire, Database, Executor};
use uuid::Uuid;

use super::dtos::CreateBlockDirectionalLinkDto;
use super::error::BlockDirectionalLinkRepositoryResult as Result;
use domain::blocks::BlockDirectionalLink;

#[async_trait]
pub trait BlockDirectionalLinkRepository<DB: Database>: Send + Sync {
    async fn get_by_id<'e, E>(&self, id: Uuid, executor: E) -> Result<Option<BlockDirectionalLink>>
    where
        E: Executor<'e, Database = DB>;

    async fn create<'e, E>(
        &self,
        input: &CreateBlockDirectionalLinkDto,
        executor: E,
    ) -> Result<BlockDirectionalLink>
    where
        E: Executor<'e, Database = DB> + Acquire<'e, Database = DB>;

    async fn delete_by_id<'e, E>(&self, id: Uuid, executor: E) -> Result<()>
    where
        E: Executor<'e, Database = DB> + Acquire<'e, Database = DB>;

    async fn delete_by_block_ids<'e, E>(
        &self,
        block_from_id: Uuid,
        block_to_id: Uuid,
        executor: E,
    ) -> Result<()>
    where
        E: Executor<'e, Database = DB> + Acquire<'e, Database = DB>;
}
