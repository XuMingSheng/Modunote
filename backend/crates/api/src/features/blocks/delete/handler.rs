use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use tracing::instrument;
use uuid::Uuid;

use super::error::DeleteBlockError;
use crate::AppState;
use storage::{Database, repositories::BlockRepository};

#[utoipa::path(
      delete,
      path = "/api/blocks/{id}",
      tag = "blocks",
    responses(
        (status = 204, description = "Block deleted successfully"),
        (status = 404, description = "Block not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument]
pub async fn delete_block(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, DeleteBlockError> {
    state.repos.blocks.delete_by_id(id, state.db.pool()).await?;

    Ok(StatusCode::NO_CONTENT)
}
