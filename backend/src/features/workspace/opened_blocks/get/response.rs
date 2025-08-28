use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OpenedBlock {
    pub block_id: Uuid,
    pub title: String,
    pub opened_at: DateTime<Utc>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenedBlocksResponse {
    pub opened_blocks: Vec<OpenedBlock>,
}
