use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{error, instrument};
use utoipa;
use uuid::Uuid;

use super::response::GetBlockParentLinksResponse;
use crate::{
    AppState,
    features::error::{ErrorResponse, HandlerError},
};
use storage::query_services::BlockLinkQueryService;

#[instrument]
#[utoipa::path(
    get,
    path = "/api/blocks/{id}/parents",
    tag = "block_links",
    responses(
        (status = StatusCode::OK, description = "List of parent blocks", body = GetBlockParentLinksResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_block_parent_links(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<GetBlockParentLinksResponse>), HandlerError> {
    let links = state
        .query_services
        .block_links
        .get_parent_blocks(id, state.db.pool())
        .await
        .map_err(|e| {
            error!("Failed to get parent links of block {id}: {e}");
            HandlerError::Anyhow
        })?;

    let response = GetBlockParentLinksResponse {
        parent_blocks: links.into_iter().map(|b| b.into()).collect(),
    };

    Ok((StatusCode::OK, Json(response)))
}
