use async_trait::async_trait;
use uuid::Uuid;

use crate::{Block, BlockCreate, BlockSummary, BlockUpdate};
use super::error::BlockResult as Result;

#[async_trait]
pub trait BlockRepositoryTrait: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Block>>;
    async fn create(&self, block_create: BlockCreate) -> Result<Block>;
    async fn update_by_id(&self, id: Uuid, block_update: &BlockUpdate) -> Result<Block>;
    async fn delete_by_id(&self, id: Uuid) -> Result<()>;

    async fn search(&self, query: &str) -> Result<Vec<BlockSummary>>;
}