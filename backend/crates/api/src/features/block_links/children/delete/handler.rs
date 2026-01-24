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
    BlockDirectionalLinkRepository,
    block_directional_link_repository::BlockDirectionalLinkRepositoryError,
};

#[instrument]
#[utoipa::path(
    delete,
    path = "/api/blocks/{id}/children/{child_id}",
    tag = "block_links",
    responses(
        (status = StatusCode::NO_CONTENT, description = "Child link deleted successfully"),
        (status = StatusCode::BAD_REQUEST, description = "Validation error"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn delete_block_child_link(
    State(state): State<Arc<AppState>>,
    Path((id, child_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, HandlerError> {
    state
        .repos
        .block_directional_links
        .delete_by_block_ids(id, child_id)
        .await
        .map_err(|e| match e {
            BlockDirectionalLinkRepositoryError::NotFoundByBlocks { .. } => HandlerError::NotFound,
            _ => {
                error!("Failed to delete block directional link {id} -> {child_id}: {e}");
                HandlerError::Anyhow
            }
        })?;

    Ok(StatusCode::NO_CONTENT)
}
