use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use storage::query_services::block_query_service::BlockSummaryDto;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BlockSummary {
    pub id: Uuid,
    pub title: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BlockSearchResponse {
    pub blocks: Vec<BlockSummary>,
}

impl From<BlockSummaryDto> for BlockSummary {
    fn from(dto: BlockSummaryDto) -> Self {
        Self {
            id: dto.id,
            title: dto.title,
        }
    }
}

impl From<Vec<BlockSummaryDto>> for BlockSearchResponse {
    fn from(blocks: Vec<BlockSummaryDto>) -> Self {
        Self {
            blocks: blocks.into_iter().map(|b| b.into()).collect(),
        }
    }
}

impl IntoResponse for BlockSearchResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
