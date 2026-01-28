use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use storage::query_services::block_query_service::OpenedBlockDto;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OpenedBlock {
    pub block_id: Uuid,
    pub title: String,
    pub opened_at: DateTime<Utc>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenedBlocksResponse {
    pub opened_blocks: Vec<OpenedBlock>,
}

impl From<OpenedBlockDto> for OpenedBlock {
    fn from(dto: OpenedBlockDto) -> Self {
        Self {
            block_id: dto.id,
            title: dto.title,
            opened_at: dto.opened_at,
        }
    }
}

impl IntoResponse for GetOpenedBlocksResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
