use async_trait::async_trait;
use uuid::Uuid;

use super::{
    dtos::{CreateBlockDto, UpdateBlockDto},
    error::BlockRepostoryResult as Result,
};
use domain::blocks::Block;

#[async_trait]
pub trait BlockRepository: Send + Sync {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Block>>;
    async fn create(&self, input: &CreateBlockDto) -> Result<Block>;
    async fn update_by_id(&self, id: Uuid, input: &UpdateBlockDto) -> Result<Block>;
    async fn delete_by_id(&self, id: Uuid) -> Result<()>;
    async fn save(&self, block: &Block) -> Result<()>;
}
