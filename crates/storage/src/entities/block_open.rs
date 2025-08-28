use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct BlockOpen {
    pub block_id: Uuid,
    pub opened_at: DateTime<Utc>,
    pub tab_order: i64,
}

#[derive(FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct BlockOpenSummary {
    pub block_id: Uuid,
    pub title: String,
    pub opened_at: DateTime<Utc>,
    pub tab_order: i64,
    pub pinned_at: Option<DateTime<Utc>>,
}
