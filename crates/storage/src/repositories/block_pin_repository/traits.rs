use async_trait::async_trait;
use uuid::Uuid;

use crate::BlockSummary;
use super::error::BlockPinResult as Result;

#[async_trait]
pub trait BlockPinRepositoryTrait: Send + Sync {
    async fn pin(&self, block_id: Uuid) -> Result<()>;
    async fn unpin(&self, block_id: Uuid) -> Result<()>;
    async fn is_pinned(&self, block_id: Uuid) -> Result<bool>;

    async fn get_pinned_summary(&self) -> Result<Vec<BlockSummary>>;
    async fn unpin_all(&self) -> Result<()>;
}