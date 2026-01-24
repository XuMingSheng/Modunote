use async_trait::async_trait;
use uuid::Uuid;

use super::error::CanvasPinResult as Result;
use crate::entities::{CanvasPin, CanvasSummary};

#[async_trait]
pub trait CanvasPinRepositoryTrait {
    async fn is_pinned(&self, canvas_id: Uuid) -> Result<bool>;
    async fn pin(&self, canvas_id: Uuid) -> Result<CanvasPin>;
    async fn unpin(&self, canvas_id: Uuid) -> Result<()>;

    async fn get_pinned_canvases(&self) -> Result<Vec<CanvasSummary>>;
    async fn unpin_all(&self) -> Result<()>;
}
