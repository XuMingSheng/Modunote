use serde;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BlockLink {
    pub block_id: Uuid,
    pub title: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockResponse {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub parent_blocks: Vec<BlockLink>,
    pub child_blocks: Vec<BlockLink>,
    pub related_blocks: Vec<BlockLink>,
}
