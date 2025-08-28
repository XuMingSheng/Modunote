use async_trait::async_trait;
use uuid::Uuid;

use crate::{BlockRelatedLink, BlockRelatedLinkCreate, BlockSummary};
use super::error::BlockRelatedLinkResult as Result;

#[async_trait]
pub trait BlockRelatedLinkRepositoryTrait: Send + Sync {
    async fn exists(&self, block_a_id: Uuid, block_b_id: Uuid) -> Result<bool>;
    async fn get_related_ids(&self, block_id: Uuid) -> Result<Vec<Uuid>>;
    async fn get_related_summary(&self, block_id: Uuid) -> Result<Vec<BlockSummary>>;
    async fn create(&self, link_create: BlockRelatedLinkCreate) -> Result<BlockRelatedLink>;
    async fn delete(&self, block_a_id: Uuid, block_b_id: Uuid) -> Result<()>;
}