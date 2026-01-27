use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use tracing::{error, instrument};

use super::{request::OpenBlockRequest, response::OpenBlockResponse};
use crate::AppState;
use crate::features::error::HandlerError;
use storage::query_services::BlockQueryService;
use storage::repositories::WorkspaceRepository;
use storage::repositories::workspace_repository::WorkspaceRepositoryError;

#[utoipa::path(
      post,
      path = "/api/workspace/opened-blocks",
      tag = "workspace",
      responses(
          (status = StatusCode::CREATED, description = "Block opened successfully", body = OpenBlockResponse),
          (status = StatusCode::NOT_FOUND, description = "Block not found"),
          (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error")
      )
  )]
#[instrument]
pub async fn open_block(
    State(state): State<Arc<AppState>>,
    Json(request): Json<OpenBlockRequest>,
) -> Result<(StatusCode, Json<OpenBlockResponse>), HandlerError> {
    let mut workspace = state.repos.workspaces.get().await.map_err(|e| {
        error!("Failed to get workspace: {e}");
        HandlerError::Anyhow
    })?;

    workspace.open_block(request.block_id);

    state
        .repos
        .workspaces
        .save(&workspace)
        .await
        .map_err(|e| match e {
            WorkspaceRepositoryError::SomeBlocksNotFound => HandlerError::NotFound,
            _ => {
                error!("Failed to save  workspace: {e}");
                HandlerError::Anyhow
            }
        })?;

    let opened_blocks = state
        .query_services
        .blocks
        .get_opened(state.db.pool())
        .await
        .map_err(|e| {
            error!("Failed to get opened blocks: {e}");
            HandlerError::Anyhow
        })?;

    let response = OpenBlockResponse {
        opened_blocks: opened_blocks.into_iter().map(|b| b.into()).collect(),
    };

    Ok((StatusCode::CREATED, Json(response)))
}
