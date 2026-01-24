use async_trait::async_trait;
use uuid::Uuid;

use super::error::CanvasResult as Result;
use crate::entities::{Canvas, CanvasCreate, CanvasSummary, CanvasUpdate};

#[async_trait]
pub trait CanvasRepositoryTrait {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Canvas>>;
    async fn create(&self, input: CanvasCreate) -> Result<Canvas>;
    async fn update_by_id(&self, id: Uuid, input: &CanvasUpdate) -> Result<Canvas>;
    async fn delete_by_id(&self, id: Uuid) -> Result<()>;
    async fn search(&self, query: &str) -> Result<Vec<CanvasSummary>>;
}
