use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum BlockRelatedLinkError {
    #[error("block_a_id and block_b_id must be distinct: {block_id}")]
    SameBlockIds { block_id: Uuid },
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BlockRelatedLink {
    pub id: Uuid,
    pub block_a_id: Uuid,
    pub block_b_id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl BlockRelatedLink {
    pub fn new(
        id: Uuid,
        block_a_id: Uuid,
        block_b_id: Uuid,
        created_at: DateTime<Utc>,
    ) -> Result<Self, BlockRelatedLinkError> {
        if block_a_id == block_b_id {
            return Err(BlockRelatedLinkError::SameBlockIds {
                block_id: block_a_id,
            });
        }

        let (block_a_id, block_b_id) = if block_a_id > block_b_id {
            (block_b_id, block_a_id)
        } else {
            (block_a_id, block_b_id)
        };

        Ok(Self {
            id,
            block_a_id,
            block_b_id,
            created_at,
        })
    }
}
