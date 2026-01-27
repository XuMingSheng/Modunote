use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{error, instrument};
use utoipa;
use uuid::Uuid;

use super::response::GetBlockChildLinksResponse;
use crate::{
    AppState,
    features::error::{ErrorResponse, HandlerError},
};
use storage::query_services::BlockLinkQueryService;

#[instrument]
#[utoipa::path(
    get,
    path = "/api/blocks/{id}/children",
    tag = "block_links",
    responses(
        (status = StatusCode::OK, description = "List of child blocks", body = GetBlockChildLinksResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_block_child_links(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<GetBlockChildLinksResponse>), HandlerError> {
    let links = state
        .query_services
        .block_links
        .get_child_blocks(id, state.db.pool())
        .await
        .map_err(|e| {
            error!("Failed to get child links of block {id}: {e}");
            HandlerError::Anyhow
        })?;

    let response = GetBlockChildLinksResponse {
        child_blocks: links.into_iter().map(|b| b.into()).collect(),
    };

    Ok((StatusCode::OK, Json(response)))
}
