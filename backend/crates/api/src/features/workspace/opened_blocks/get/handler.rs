use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use tracing::{error, instrument};

use super::GetOpenedBlocksResponse;
use crate::AppState;
use storage::query_services::BlockQueryService;

#[utoipa::path(
      get,
      path = "/api/workspace/opened-blocks",
      tag = "workspace",
      responses(
          (status = 200, description = "List of opened blocks", body = GetOpenedBlocksResponse),
          (status = 500, description = "Internal server error")
      )
  )]
#[instrument]
pub async fn get_opened_blocks(
    State(state): State<Arc<AppState>>,
) -> Result<Json<GetOpenedBlocksResponse>, StatusCode> {
    let opened_blocks = state
        .query_services
        .blocks
        .get_opened(state.db.pool())
        .await
        .map_err(|e| {
            error!("Failed to get opened blocks {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let response = GetOpenedBlocksResponse {
        opened_blocks: opened_blocks.into_iter().map(|b| b.into()).collect(),
    };

    Ok(Json(response))
}
