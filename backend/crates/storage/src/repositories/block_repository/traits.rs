use async_trait::async_trait;
use sqlx::{Acquire, Database, Executor};
use uuid::Uuid;

use super::error::BlockRepostoryResult as Result;
use domain::blocks::Block;

#[async_trait]
pub trait BlockRepository<DB: Database>: Send + Sync {
    async fn get_by_id<'e, E>(&self, id: Uuid, executor: E) -> Result<Option<Block>>
    where
        E: Executor<'e, Database = DB>;

    async fn delete_by_id<'e, E>(&self, id: Uuid, executor: E) -> Result<()>
    where
        E: Executor<'e, Database = DB> + Acquire<'e, Database = DB>;

    async fn save<'e, E>(&self, block: &Block, executor: E) -> Result<()>
    where
        E: Executor<'e, Database = DB>;
}
