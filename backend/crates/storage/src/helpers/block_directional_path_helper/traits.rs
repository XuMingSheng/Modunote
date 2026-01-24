use std::collections::HashSet;

use async_trait::async_trait;
use sqlx::{Database, Transaction};
use uuid::Uuid;

use super::error::BlockDirectionalPathHelperResult as Result;

#[async_trait]
pub trait BlockDirectionalPathHelperTrait<Db: Database>: Send + Sync {
    // Path existence and relationship queries
    async fn is_ancestor_descendant(&self, ancestor_id: Uuid, descendant_id: Uuid) -> Result<bool>;

    // Single block hierarchy queries
    async fn get_ancestors_ids(&self, block_id: Uuid) -> Result<Vec<Uuid>>;

    async fn get_descendants_ids(&self, block_id: Uuid) -> Result<Vec<Uuid>>;

    // Path enumeration queries
    async fn get_paths_from_id(&self, ancestor_id: Uuid) -> Result<Vec<Vec<Uuid>>>;

    async fn get_paths_to_id(&self, descendant_id: Uuid) -> Result<Vec<Vec<Uuid>>>;

    async fn get_paths_between_ids(
        &self,
        ancestor_id: Uuid,
        descendant_id: Uuid,
    ) -> Result<Vec<Vec<Uuid>>>;

    async fn get_paths_between_id_sets(
        &self,
        ancestor_ids: HashSet<Uuid>,
        descendant_ids: HashSet<Uuid>,
    ) -> Result<Vec<Vec<Uuid>>>;

    // Path maintenance operations (require transaction)
    async fn create_paths_for_new_link(
        &self,
        from_id: Uuid,
        to_id: Uuid,
        transaction: &mut Transaction<'_, Db>,
    ) -> Result<()>;

    async fn delete_paths_using_link(
        &self,
        from_id: Uuid,
        to_id: Uuid,
        transaction: &mut Transaction<'_, Sqlite>,
    ) -> Result<()>;

    async fn delete_paths_using_block(
        &self,
        block_id: Uuid,
        transaction: &mut Transaction<'_, Sqlite>,
    ) -> Result<()>;
}
