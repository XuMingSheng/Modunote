use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use storage::query_services::block_query_service::OpenedBlockDto;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OpenedBlock {
    pub id: Uuid,
    pub title: String,
    pub opened_at: DateTime<Utc>,
    pub tab_order: u32,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OpenBlockResponse {
    pub opened_blocks: Vec<OpenedBlock>,
}

impl From<OpenedBlockDto> for OpenedBlock {
    fn from(dto: OpenedBlockDto) -> Self {
        Self {
            id: dto.id,
            title: dto.title,
            opened_at: dto.opened_at,
            tab_order: dto.tab_order,
        }
    }
}
