use std::sync::Arc;

use axum::{Json, extract::State};
use tracing::instrument;

use super::{error::OpenBlockError, request::OpenBlockRequest, response::OpenBlockResponse};
use crate::AppState;
use storage::{Database, repositories::WorkspaceRepository};

#[utoipa::path(
      post,
      path = "/api/workspace/opened-blocks",
      tag = "workspace",
      responses(
          (status = 201, description = "Block opened successfully", body = OpenBlockResponse),
          (status = 404, description = "Block not found"),
          (status = 500, description = "Internal server error")
      )
  )]
#[instrument]
pub async fn open_block(
    State(state): State<Arc<AppState>>,
    Json(request): Json<OpenBlockRequest>,
) -> Result<OpenBlockResponse, OpenBlockError> {
    let mut workspace = state.repos.workspaces.get(state.db.pool()).await?;

    workspace.open_block(request.block_id);

    state
        .repos
        .workspaces
        .save(&workspace, state.db.pool())
        .await?;

    let opened_block = workspace
        .opened_blocks
        .iter()
        .find(|b| b.block_id == request.block_id)
        .ok_or(OpenBlockError::OpenedBlockMissing)?;

    let response = OpenBlockResponse {
        block_id: opened_block.block_id,
        opened_at: opened_block.opened_at,
    };

    Ok(response)
}
