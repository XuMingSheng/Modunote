use async_trait::async_trait;
use uuid::Uuid;

// use super::dtos::CreateBlockDirectionalLinkDto;
use super::dtos::CreateBlockRelatedLinkDto;
use super::error::BlockRelatedLinkResult as Result;
use domain::blocks::BlockRelatedLink;

#[async_trait]
pub trait BlockRelatedLinkRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<BlockRelatedLink>>;
    async fn create(&self, input: &CreateBlockRelatedLinkDto) -> Result<BlockRelatedLink>;
    async fn delete_by_id(&self, id: Uuid) -> Result<()>;
    async fn delete_by_block_ids(&self, block_a_id: Uuid, block_b_id: Uuid) -> Result<()>;
}
