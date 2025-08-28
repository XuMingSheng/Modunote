use axum::{Json, extract::State, http::StatusCode};
use tracing::{error, instrument};

use super::response::{GetOpenedBlocksResponse, OpenedBlock};
use storage::{BlockOpenRepositoryTrait, RepositoryProvider};

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
    State(repos): State<RepositoryProvider>,
) -> Result<Json<GetOpenedBlocksResponse>, StatusCode> {
    let mut opened_summaries = repos.block_opens.get_opened_summary().await.map_err(|e| {
        error!("Failed to get opened blocks: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    opened_summaries.sort_by_key(|b| b.tab_order);

    let opened_blocks: Vec<OpenedBlock> = opened_summaries
        .into_iter()
        .map(|block| OpenedBlock {
            block_id: block.block_id,
            title: block.title,
            opened_at: block.opened_at,
        })
        .collect();

    let response = GetOpenedBlocksResponse { opened_blocks };
    Ok(Json(response))
}
