use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{error, instrument};
use uuid::Uuid;

use crate::AppState;
use storage::repositories::WorkspaceRepository;

#[utoipa::path(
      delete,
      path = "/api/workspace/opened-blocks/{block_id}",
      tag = "workspace",
      params(
          ("block_id" = uuid::Uuid, Path, description = "Block ID to close")
      ),
      responses(
          (status = 204, description = "Block closed successfully"),
          (status = 404, description = "Block is not opened"),
          (status = 500, description = "Internal server error")
      )
  )]
#[instrument]
pub async fn close_block(
    State(state): State<Arc<AppState>>,
    Path(block_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let mut workspace = state.repos.workspaces.get().await.map_err(|e| {
        error!("Failed to get workspace: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let is_opened = workspace
        .opened_blocks
        .iter()
        .any(|b| b.block_id == block_id);

    if !is_opened {
        return Err(StatusCode::NOT_FOUND);
    }

    workspace.close_block(block_id);

    state.repos.workspaces.save(&workspace).await.map_err(|e| {
        error!("Failed to save workspace: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::NO_CONTENT)
}
