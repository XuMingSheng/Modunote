use async_trait::async_trait;
use uuid::Uuid;

use super::error::BlockPinResult as Result;
use crate::entities::{BlockPin, BlockSummary};

#[async_trait]
pub trait BlockPinRepositoryTrait: Send + Sync {
    async fn is_pinned(&self, block_id: Uuid) -> Result<bool>;
    async fn pin(&self, block_id: Uuid) -> Result<BlockPin>;
    async fn unpin(&self, block_id: Uuid) -> Result<()>;

    async fn get_pinned_blocks(&self) -> Result<Vec<BlockSummary>>;
    async fn unpin_all(&self) -> Result<()>;
}
