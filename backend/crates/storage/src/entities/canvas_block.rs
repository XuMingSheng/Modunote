use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct CanvasBlock {
    pub id: Uuid,
    pub canvas_id: Uuid,
    pub block_id: Uuid,
    pub grid_x: i32,
    pub grid_y: i32,
    pub grid_width: i32,
    pub grid_height: i32,
    pub z: i32,
    pub scale: f64,
    pub content_visible: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct BlockInCanvas {
    // Canvas positioning
    pub id: Uuid,
    pub canvas_id: Uuid,
    pub block_id: Uuid,
    pub grid_x: i32,
    pub grid_y: i32,
    pub grid_width: i32,
    pub grid_height: i32,
    pub z: i32,
    pub scale: f64,
    pub content_visible: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Block content
    pub block_title: String,
    pub block_content: String,
}

#[derive(Debug)]
pub struct CanvasBlockCreate {
    pub canvas_id: Uuid,
    pub block_id: Uuid,
    pub grid_x: i32,
    pub grid_y: i32,
    pub grid_width: i32,
    pub grid_height: i32,
}

#[derive(Debug)]
pub struct CanvasBlockUpdate {
    pub grid_x: Option<i32>,
    pub grid_y: Option<i32>,
    pub grid_width: Option<i32>,
    pub grid_height: Option<i32>,
    pub z: Option<i32>,
    pub scale: Option<f64>,
    pub content_visible: Option<bool>,
}
