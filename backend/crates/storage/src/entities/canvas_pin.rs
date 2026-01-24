use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct CanvasPin {
    pub canvas_id: Uuid,
    pub pinned_at: DateTime<Utc>,
}
