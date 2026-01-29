use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use tracing::instrument;
use uuid::Uuid;

use super::error::CloseBlockError;
use crate::AppState;
use storage::Database;
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
) -> Result<StatusCode, CloseBlockError> {
    let mut workspace = state.repos.workspaces.get(state.db.pool()).await?;

    let is_opened = workspace
        .opened_blocks
        .iter()
        .any(|b| b.block_id == block_id);

    if !is_opened {
        return Err(CloseBlockError::NotOpened);
    }

    workspace.close_block(block_id);

    state.repos.workspaces.save(&workspace, state.db.pool()).await?;

    Ok(StatusCode::NO_CONTENT)
}
