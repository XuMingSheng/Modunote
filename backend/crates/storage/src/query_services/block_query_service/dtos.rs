use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Clone, Debug)]
pub struct BlockSummaryDto {
    pub id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub opened_at: Option<DateTime<Utc>>,
}

pub struct OpenedBlockDto {
    pub id: Uuid,
    pub title: String,
    pub opened_at: DateTime<Utc>,
    pub tab_index: u32,
}
