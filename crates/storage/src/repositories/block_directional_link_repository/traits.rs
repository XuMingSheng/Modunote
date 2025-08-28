use std::collections::HashSet;

use async_trait::async_trait;
use uuid::Uuid;

use super::error::BlockDirectionalLinkResult as Result;
use crate::{BlockDirectionalLink, BlockDirectionalLinkCreate, BlockSummary};

#[async_trait]
pub trait BlockDirectionalLinkRepositoryTrait: Send + Sync {
    // Basic CRUD
    async fn get_children_ids(&self, block_id: Uuid) -> Result<Vec<Uuid>>;
    async fn get_parents_ids(&self, block_id: Uuid) -> Result<Vec<Uuid>>;
    async fn get_children_summary(&self, block_id: Uuid) -> Result<Vec<BlockSummary>>;
    async fn get_parents_summary(&self, block_id: Uuid) -> Result<Vec<BlockSummary>>;
    async fn exists(&self, block_from_id: Uuid, block_to_id: Uuid) -> Result<bool>;

    async fn create(&self, link_create: BlockDirectionalLinkCreate)
    -> Result<BlockDirectionalLink>;

    async fn delete(&self, block_from_id: Uuid, block_to_id: Uuid) -> Result<()>;

    // DAG structure queries
    async fn is_ancestor_descendant(&self, ancestor_id: Uuid, descendant_id: Uuid) -> Result<bool>;
    async fn get_descendants_ids(&self, block_id: Uuid) -> Result<Vec<Uuid>>;
    async fn get_ancestors_ids(&self, block_id: Uuid) -> Result<Vec<Uuid>>;
    async fn get_root_ids(&self) -> Result<Vec<Uuid>>;
    async fn get_leaf_ids(&self) -> Result<Vec<Uuid>>;

    // DAG path queries
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
}
