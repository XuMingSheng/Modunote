use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use tracing::{error, instrument};

use super::{request::BlockSearchRequest, response::BlockSearchResponse};
use crate::{AppState, features::error::HandlerError};
use storage::query_services::BlockQueryService;

#[utoipa::path(
    post,
    path = "/api/search/blocks",
    tag = "search",
    responses(
        (status = StatusCode::OK, description = "Success", body = BlockSearchResponse),
        (status = StatusCoee::INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
#[instrument]
pub async fn search_blocks(
    State(state): State<Arc<AppState>>,
    Json(request): Json<BlockSearchRequest>,
) -> Result<(StatusCode, Json<BlockSearchResponse>), HandlerError> {
    let blocks = state
        .query_services
        .blocks
        .search(&request.query)
        .await
        .map_err(|e| {
            error!("Failed to search blocks by '{}': {e}", request.query);
            HandlerError::Anyhow
        })?;

    let response: BlockSearchResponse = blocks.into();

    Ok((StatusCode::OK, Json(response)))
}
