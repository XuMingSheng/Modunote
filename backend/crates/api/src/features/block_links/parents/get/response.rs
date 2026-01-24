use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use storage::query_services::block_link_query_service::LinkedBlockDto;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ParentBlock {
    pub block_id: Uuid,
    pub title: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetBlockParentLinksResponse {
    pub parent_blocks: Vec<ParentBlock>,
}

impl From<LinkedBlockDto> for ParentBlock {
    fn from(block: LinkedBlockDto) -> Self {
        Self {
            block_id: block.block_id,
            title: block.title,
        }
    }
}
