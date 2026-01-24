use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Canvas {
    pub id: Uuid,
    pub name: String,
    pub viewport_x: f64,
    pub viewport_y: f64,
    pub zoom_level: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
