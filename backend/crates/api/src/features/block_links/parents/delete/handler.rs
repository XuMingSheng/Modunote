use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use tracing::instrument;
use utoipa;
use uuid::Uuid;

use super::error::{DeleteBlockParentLinkError, ErrorResponse};
use crate::AppState;
use storage::Database;
use storage::repositories::BlockDirectionalLinkRepository;

#[instrument]
#[utoipa::path(
    delete,
    path = "/api/blocks/{id}/parents/{parent_id}",
    tag = "block_links",
    responses(
        (status = 204, description = "Parent link deleted successfully"),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn delete_block_parent_link(
    State(state): State<Arc<AppState>>,
    Path((id, parent_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, DeleteBlockParentLinkError> {
    state
        .repos
        .block_directional_links
        .delete_by_block_ids(parent_id, id, state.db.pool())
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
