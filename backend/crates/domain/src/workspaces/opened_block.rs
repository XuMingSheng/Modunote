use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct OpenedBlock {
    pub block_id: Uuid,
    pub opened_at: DateTime<Utc>,
    pub tab_index: usize,
}
