use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use domain::blocks::Block;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateBlockResponse {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl From<Block> for CreateBlockResponse {
    fn from(block: Block) -> Self {
        Self {
            id: block.id,
            title: block.title,
            content: block.content,
            created_at: block.created_at,
        }
    }
}

impl IntoResponse for CreateBlockResponse {
    fn into_response(self) -> Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}
