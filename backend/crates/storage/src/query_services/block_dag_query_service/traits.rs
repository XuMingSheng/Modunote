use std::collections::HashSet;

use async_trait::async_trait;
use uuid::Uuid;

use super::dto::BlockSummaryDto;
use super::error::BlockDagQueryServiceResult as Result;

#[async_trait]
pub trait BlockDagQueryService: Send + Sync {
    // node queries
    async fn is_ancestor_descendant(&self, ancestor_id: Uuid, descendant_id: Uuid) -> Result<bool>;
    async fn get_descendants_ids(&self, block_id: Uuid) -> Result<Vec<Uuid>>;
    async fn get_ancestors_ids(&self, block_id: Uuid) -> Result<Vec<Uuid>>;
    async fn get_root_ids(&self) -> Result<Vec<Uuid>>;
    async fn get_leaf_ids(&self) -> Result<Vec<Uuid>>;

    async fn get_descendants(&self, block_id: Uuid) -> Result<BlockSummaryDto>;
    async fn get_ancestors(&self, block_id: Uuid) -> Result<BlockSummaryDto>;
    async fn get_roots(&self, block_id: Uuid) -> Result<BlockSummaryDto>;
    async fn get_leaves(&self, block_id: Uuid) -> Result<BlockSummaryDto>;

    // path queries
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
