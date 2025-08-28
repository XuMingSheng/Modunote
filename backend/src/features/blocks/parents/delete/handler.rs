use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{error, instrument};
use uuid::Uuid;

use storage::{BlockDirectionalLinkError, BlockDirectionalLinkRepositoryTrait, RepositoryProvider};

#[utoipa::path(
    delete,
    path = "/api/blocks/{child_id}/parents/{parent_id}",
    tag = "blocks",
    params(
        ("child_id" = uuid::Uuid, Path, description = "Child block ID"),
        ("parent_id" = uuid::Uuid, Path, description = "Parent block ID to unlink")
    ),
    responses(
        (status = 204, description = "Parent link deleted successfully"),
        (status = 404, description = "Link not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument]
pub async fn delete_parent_link(
    State(repos): State<RepositoryProvider>,
    Path((child_id, parent_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, StatusCode> {
    repos
        .block_directional_links
        .delete(parent_id, child_id)
        .await
        .map_err(|e| match e {
            BlockDirectionalLinkError::NotFound { .. } => StatusCode::NOT_FOUND,
            _ => {
                error!("Failed to delete parent link: {e}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;

    Ok(StatusCode::NO_CONTENT)
}
