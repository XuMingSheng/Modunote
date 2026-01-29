use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use tracing::instrument;
use utoipa;
use uuid::Uuid;

use super::error::{DeleteBlockRelatedLinkError, ErrorResponse};
use crate::AppState;
use storage::Database;
use storage::repositories::BlockRelatedLinkRepository;

#[instrument]
#[utoipa::path(
    delete,
    path = "/api/blocks/{id}/related/{related_id}",
    tag = "block_links",
    responses(
        (status = 204, description = "Related link deleted successfully"),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn delete_block_related_link(
    State(state): State<Arc<AppState>>,
    Path((id, related_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, DeleteBlockRelatedLinkError> {
    state
        .repos
        .block_related_links
        .delete_by_block_ids(id, related_id, state.db.pool())
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
