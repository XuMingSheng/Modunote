use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{error, instrument};
use uuid::Uuid;

use storage::{BlockRelatedLinkError, BlockRelatedLinkRepositoryTrait, RepositoryProvider};

#[utoipa::path(
      delete,
      path = "/api/blocks/{id}/related-links/{related_id}",
      tag = "blocks",
      params(
          ("id" = uuid::Uuid, Path, description = "Block ID"),
          ("related_id" = uuid::Uuid, Path, description = "Related block ID to unlink")
      ),
      responses(
          (status = 204, description = "Related link deleted successfully"),
          (status = 404, description = "Link not found"),
          (status = 500, description = "Internal server error")
      )
  )]
#[instrument]
pub async fn delete_related_link(
    State(repos): State<RepositoryProvider>,
    Path((id, related_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, StatusCode> {
    repos
        .block_related_links
        .delete(id, related_id)
        .await
        .map_err(|e| match e {
            BlockRelatedLinkError::NotFound { .. } => StatusCode::NOT_FOUND,
            _ => {
                error!("Failed to delete related link: {e}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;

    Ok(StatusCode::NO_CONTENT)
}
