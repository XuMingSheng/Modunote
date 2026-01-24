use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use storage::query_services::block_link_query_service::LinkedBlockDto;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ChildBlock {
    pub block_id: Uuid,
    pub title: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetBlockChildLinksResponse {
    pub child_blocks: Vec<ChildBlock>,
}

impl From<LinkedBlockDto> for ChildBlock {
    fn from(block: LinkedBlockDto) -> Self {
        Self {
            block_id: block.block_id,
            title: block.title,
        }
    }
}
