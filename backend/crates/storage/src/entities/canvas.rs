use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct Canvas {
    pub id: Uuid,
    pub name: String,
    pub viewport_x: f64,
    pub viewport_y: f64,
    pub zoom_level: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct CanvasSummary {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct CanvasCreate {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug)]
pub struct CanvasUpdate {
    pub name: Option<String>,
    pub viewport_x: Option<f64>,
    pub viewport_y: Option<f64>,
    pub zoom_level: Option<f64>,
}
