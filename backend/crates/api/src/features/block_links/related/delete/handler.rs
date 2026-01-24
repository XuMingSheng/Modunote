use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{error, instrument};
use utoipa;
use uuid::Uuid;

use crate::{
    AppState,
    features::error::{ErrorResponse, HandlerError},
};
use storage::repositories::{
    BlockRelatedLinkRepository,
    block_related_link_repository::BlockRelatedLinkError,
};

#[instrument]
#[utoipa::path(
    delete,
    path = "/api/blocks/{id}/related/{related_id}",
    tag = "block_links",
    responses(
        (status = StatusCode::NO_CONTENT, description = "Related link deleted successfully"),
        (status = StatusCode::BAD_REQUEST, description = "Validation error"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn delete_block_related_link(
    State(state): State<Arc<AppState>>,
    Path((id, related_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, HandlerError> {
    state
        .repos
        .block_related_links
        .delete_by_block_ids(id, related_id)
        .await
        .map_err(|e| match e {
            BlockRelatedLinkError::NotFoundByBlocks { .. } => HandlerError::NotFound,
            _ => {
                error!("Failed to delete related link for blocks {id} <-> {related_id}: {e}");
                HandlerError::Anyhow
            }
        })?;

    Ok(StatusCode::NO_CONTENT)
}
