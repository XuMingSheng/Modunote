use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct BlockDirectionalLink {
    pub id: Uuid,
    pub block_from_id: Uuid,
    pub block_to_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct BlockDirectionalLinkCreate {
    pub id: Uuid,
    pub block_from_id: Uuid,
    pub block_to_id: Uuid,
}
