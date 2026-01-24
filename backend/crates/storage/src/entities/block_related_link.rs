use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct BlockRelatedLink {
    pub id: Uuid,
    pub block_a_id: Uuid,
    pub block_b_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct BlockRelatedLinkCreate {
    pub id: Uuid,
    pub block_a_id: Uuid,
    pub block_b_id: Uuid,
}
