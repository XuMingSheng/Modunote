use async_trait::async_trait;
use uuid::Uuid;

use super::dtos::{AllLinkedBlocksDto, LinkedBlockDto};
use super::error::BlockLinkQueryServiceResult as Result;

#[async_trait]
pub trait BlockLinkQueryService: Send + Sync {
    async fn get_linked_blocks(&self, block_id: Uuid) -> Result<AllLinkedBlocksDto>;
    async fn get_parent_blocks(&self, block_id: Uuid) -> Result<Vec<LinkedBlockDto>>;
    async fn get_child_blocks(&self, block_id: Uuid) -> Result<Vec<LinkedBlockDto>>;
    async fn get_related_blocks(&self, block_id: Uuid) -> Result<Vec<LinkedBlockDto>>;
}
