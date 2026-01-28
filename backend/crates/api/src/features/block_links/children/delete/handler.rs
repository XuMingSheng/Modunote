use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use tracing::instrument;
use utoipa;
use uuid::Uuid;

use super::error::{DeleteBlockChildLinkError, ErrorResponse};
use crate::AppState;
use storage::Database;
use storage::repositories::BlockDirectionalLinkRepository;

#[instrument]
#[utoipa::path(
    delete,
    path = "/api/blocks/{id}/children/{child_id}",
    tag = "block_links",
    responses(
        (status = 204, description = "Child link deleted successfully"),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn delete_block_child_link(
    State(state): State<Arc<AppState>>,
    Path((id, child_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, DeleteBlockChildLinkError> {
    state
        .repos
        .block_directional_links
        .delete_by_block_ids(id, child_id, state.db.pool())
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
