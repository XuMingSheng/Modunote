use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{error, instrument};
use uuid::Uuid;

use super::response::GetBlockResponse;
use crate::{AppState, features::error::HandlerError};
use storage::query_services::BlockLinkQueryService;
use storage::repositories::BlockRepository;

#[instrument]
#[utoipa::path(
    get,
    path = "/api/blocks/{id}",
    tag = "blocks",
    responses(
        (status = StatusCode::OK, description = "Block found", body = GetBlockResponse),
        (status = StatusCode::NOT_FOUND, description = "Block not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
pub async fn get_block(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<GetBlockResponse>), HandlerError> {
    let block = state
        .repos
        .blocks
        .get_by_id(id)
        .await
        .map_err(|e| {
            error!("Failed to get block {id}, {e}");
            HandlerError::Anyhow
        })?
        .ok_or(HandlerError::NotFound)?;

    let linked_blocks = state
        .query_services
        .block_links
        .get_linked_blocks(id, state.db.pool())
        .await
        .map_err(|e| {
            error!("Failed to get linked blocks, {id}, {e}");
            HandlerError::Anyhow
        })?;

    let response = GetBlockResponse::from_block_and_linked(block, linked_blocks);

    Ok((StatusCode::OK, Json(response)))
}
