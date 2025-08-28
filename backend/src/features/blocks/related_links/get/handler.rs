use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{error, instrument};
use uuid::Uuid;

use super::response::{GetRelatedBlocksResponse, RelatedBlock};
use storage::{BlockRelatedLinkRepositoryTrait, BlockRepositoryTrait, RepositoryProvider};

#[utoipa::path(
      get,
      path = "/api/blocks/{id}/related-links",
      tag = "blocks",
      params(
          ("id" = uuid::Uuid, Path, description = "Block ID")
      ),
      responses(
          (status = 200, description = "List of related blocks", body = GetRelatedBlocksResponse),
          (status = 404, description = "Block not found"),
          (status = 500, description = "Internal server error")
      )
  )]
#[instrument]
pub async fn get_related_blocks(
    State(repos): State<RepositoryProvider>,
    Path(id): Path<Uuid>,
) -> Result<Json<GetRelatedBlocksResponse>, StatusCode> {
    repos
        .blocks
        .get_by_id(id)
        .await
        .map_err(|e| {
            error!("Failed to get block {}: {e}", id);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    let related_summaries = repos
        .block_related_links
        .get_related_summary(id)
        .await
        .map_err(|e| {
            error!("Failed to get related blocks for block {}: {e}", id);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let related_blocks = related_summaries
        .into_iter()
        .map(|summary| RelatedBlock {
            block_id: summary.id,
            title: summary.title,
        })
        .collect();

    let response = GetRelatedBlocksResponse { related_blocks };

    Ok(Json(response))
}
