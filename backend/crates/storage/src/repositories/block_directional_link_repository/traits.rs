use async_trait::async_trait;
use uuid::Uuid;

use super::dtos::CreateBlockDirectionalLinkDto;
use super::error::BlockDirectionalLinkRepositoryResult as Result;
use domain::blocks::BlockDirectionalLink;

#[async_trait]
pub trait BlockDirectionalLinkRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<BlockDirectionalLink>>;
    async fn create(&self, input: &CreateBlockDirectionalLinkDto) -> Result<BlockDirectionalLink>;
    async fn delete_by_id(&self, id: Uuid) -> Result<()>;
    async fn delete_by_block_ids(&self, block_from_id: Uuid, block_to_id: Uuid) -> Result<()>;
}
