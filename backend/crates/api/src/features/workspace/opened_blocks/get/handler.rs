use std::sync::Arc;

use axum::extract::State;
use tracing::instrument;

use super::error::GetOpenedBlockError;
use super::response::GetOpenedBlocksResponse;
use crate::AppState;
use storage::Database;
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
) -> Result<GetOpenedBlocksResponse, GetOpenedBlockError> {
    let opened_blocks = state
        .query_services
        .blocks
        .get_opened(state.db.pool())
        .await?;

    let response = GetOpenedBlocksResponse {
        opened_blocks: opened_blocks.into_iter().map(|b| b.into()).collect(),
    };

    Ok(response)
}
