use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct OpenedBlockDto {
    pub id: Uuid,
    pub opened_at: DateTime<Utc>,
    pub tab_index: usize,
    pub title: String,
}

#[derive(FromRow, Clone, Debug)]
pub struct BlockSummaryDto {
    pub id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub opened_at: Option<DateTime<Utc>>,
}

#[derive(FromRow, Clone, Debug)]
pub struct BlockExportDto {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
