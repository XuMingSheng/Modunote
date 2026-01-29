use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum BlockDirectionalLinkError {
    #[error("block_from_id and block_to_id must be distinct: {block_id}")]
    SameBlockIds { block_id: Uuid },
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BlockDirectionalLink {
    pub id: Uuid,
    pub block_from_id: Uuid,
    pub block_to_id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl BlockDirectionalLink {
    pub fn new(block_from_id: Uuid, block_to_id: Uuid) -> Result<Self, BlockDirectionalLinkError> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        if block_from_id == block_to_id {
            return Err(BlockDirectionalLinkError::SameBlockIds {
                block_id: block_from_id,
            });
        }

        Ok(Self {
            id,
            block_from_id,
            block_to_id,
            created_at: now,
        })
    }
}
