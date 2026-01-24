use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct LinkedBlockDto {
    pub link_id: Uuid,
    pub block_id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct AllLinkedBlocksDto {
    pub parent_blocks: Vec<LinkedBlockDto>,
    pub child_blocks: Vec<LinkedBlockDto>,
    pub related_blocks: Vec<LinkedBlockDto>,
}
