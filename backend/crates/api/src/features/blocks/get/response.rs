use serde;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use domain::blocks::Block;
use storage::query_services::block_link_query_service::{AllLinkedBlocksDto, LinkedBlockDto};

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LinkedBlock {
    pub block_id: Uuid,
    pub link_id: Uuid,
    pub title: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockResponse {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub parent_blocks: Vec<LinkedBlock>,
    pub child_blocks: Vec<LinkedBlock>,
    pub related_blocks: Vec<LinkedBlock>,
}

impl From<LinkedBlockDto> for LinkedBlock {
    fn from(dto: LinkedBlockDto) -> Self {
        Self {
            block_id: dto.block_id,
            link_id: dto.link_id,
            title: dto.title,
        }
    }
}

impl GetBlockResponse {
    pub fn from_block_and_linked(block: Block, linked_blocks: AllLinkedBlocksDto) -> Self {
        let parent_blocks: Vec<LinkedBlock> = linked_blocks
            .parent_blocks
            .into_iter()
            .map(|b| b.into())
            .collect();
        let child_blocks: Vec<LinkedBlock> = linked_blocks
            .child_blocks
            .into_iter()
            .map(|b| b.into())
            .collect();
        let related_blocks: Vec<LinkedBlock> = linked_blocks
            .related_blocks
            .into_iter()
            .map(|b| b.into())
            .collect();

        Self {
            id: block.id,
            title: block.title,
            content: block.content,
            parent_blocks,
            child_blocks,
            related_blocks,
        }
    }
}
