use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct BlockSummaryDto {
    pub id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
