use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{error, instrument};
use utoipa;
use uuid::Uuid;

use super::response::GetBlockRelatedLinksResponse;
use crate::{
    AppState,
    features::error::{ErrorResponse, HandlerError},
};
use storage::query_services::BlockLinkQueryService;

#[instrument]
#[utoipa::path(
    get,
    path = "/api/blocks/{id}/related",
    tag = "block_links",
    responses(
        (status = StatusCode::OK, description = "List of related blocks", body = GetBlockRelatedLinksResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_block_related_links(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<GetBlockRelatedLinksResponse>), HandlerError> {
    let links = state
        .query_services
        .block_links
        .get_related_blocks(id, state.db.pool())
        .await
        .map_err(|e| {
            error!("Failed to get related links of block {id}: {e}");
            HandlerError::Anyhow
        })?;

    let response = GetBlockRelatedLinksResponse {
        related_blocks: links.into_iter().map(|b| b.into()).collect(),
    };

    Ok((StatusCode::OK, Json(response)))
}
