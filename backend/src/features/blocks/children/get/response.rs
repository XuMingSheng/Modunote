use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ChildBlock {
    pub block_id: Uuid,
    pub title: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetChildBlocksResponse {
    pub child_blocks: Vec<ChildBlock>,
}
