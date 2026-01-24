use async_trait::async_trait;

use super::{
    dtos::{BlockSummaryDto, OpenedBlockDto},
    error::BlockQueryServiceResult as Result,
};

#[async_trait]
pub trait BlockQueryService: Send + Sync {
    async fn get_opened(&self) -> Result<Vec<OpenedBlockDto>>;
    async fn search(&self, query: &str) -> Result<Vec<BlockSummaryDto>>;
}
