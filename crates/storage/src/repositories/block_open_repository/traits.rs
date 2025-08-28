use async_trait::async_trait;
use uuid::Uuid;

use crate::{BlockOpen, BlockOpenSummary};
use super::error::BlockOpenResult as Result;

#[async_trait]
pub trait BlockOpenRepositoryTrait: Send + Sync {
    async fn open(&self, block_id: Uuid) -> Result<BlockOpen>;
    async fn close(&self, block_id: Uuid) -> Result<()>;
    async fn is_opened(&self, block_id: Uuid) -> Result<bool>;

    async fn get_opened_summary(&self) -> Result<Vec<BlockOpenSummary>>;
    async fn close_all(&self) -> Result<()>;
}