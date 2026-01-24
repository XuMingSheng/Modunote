use async_trait::async_trait;
use uuid::Uuid;

use super::error::CanvasBlockResult;
use crate::entities::{BlockInCanvas, CanvasBlock, CanvasBlockCreate, CanvasBlockUpdate};

#[async_trait]
pub trait CanvasBlockRepositoryTrait {
    async fn get_by_canvas_id(&self, canvas_id: Uuid) -> CanvasBlockResult<Vec<CanvasBlock>>;
    async fn create(
        &self,
        canvas_block_create: CanvasBlockCreate,
    ) -> CanvasBlockResult<CanvasBlock>;
    async fn update(
        &self,
        id: Uuid,
        canvas_block_update: CanvasBlockUpdate,
    ) -> CanvasBlockResult<CanvasBlock>;
    async fn delete(&self, id: Uuid) -> CanvasBlockResult<()>;

    async fn get_blocks_in_canvas(&self, canvas_id: Uuid) -> CanvasBlockResult<Vec<BlockInCanvas>>;
}
