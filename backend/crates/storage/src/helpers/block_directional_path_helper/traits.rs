use async_trait::async_trait;
use sqlx::{Acquire, Database, Executor};
use uuid::Uuid;

use super::error::BlockDirectionalPathHelperResult as Result;

#[async_trait]
pub trait BlockDirectionalPathHelper<DB: Database>: Send + Sync {
    async fn is_ancestor_descendant<'e, E>(
        &self,
        ancestor_id: Uuid,
        descendant_id: Uuid,
        executor: E,
    ) -> Result<bool>
    where
        E: Executor<'e, Database = DB>;

    async fn create_paths_for_link<'e, E>(
        &self,
        from_id: Uuid,
        to_id: Uuid,
        executor: E,
    ) -> Result<()>
    where
        E: Executor<'e, Database = DB> + Acquire<'e, Database = DB>;

    async fn delete_paths_using_link<'e, E>(
        &self,
        from_id: Uuid,
        to_id: Uuid,
        executor: E,
    ) -> Result<()>
    where
        E: Executor<'e, Database = DB>;

    async fn delete_paths_using_block<'e, E>(&self, block_id: Uuid, executor: E) -> Result<()>
    where
        E: Executor<'e, Database = DB>;
}
