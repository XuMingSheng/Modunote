use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{error, instrument};
use uuid::Uuid;

use super::{request::CreateChildLinkRequest, response::CreateChildLinkResponse};
use storage::{
    BlockDirectionalLinkCreate, BlockDirectionalLinkError, BlockDirectionalLinkRepositoryTrait,
    BlockRepositoryTrait, RepositoryProvider,
};

#[utoipa::path(
      post,
      path = "/api/blocks/{id}/children",
      tag = "blocks",
      params(
          ("id" = uuid::Uuid, Path, description = "Parent block ID")
      ),
      request_body = CreateChildLinkRequest,
      responses(
          (status = 201, description = "Child link created successfully", body = CreateChildLinkResponse),
          (status = 404, description = "Block not found"),
          (status = 409, description = "Link already exists or would create cycle"),
          (status = 500, description = "Internal server error")
      )
  )]
#[instrument]
pub async fn create_child_link(
    State(repos): State<RepositoryProvider>,
    Path(parent_id): Path<Uuid>,
    Json(request): Json<CreateChildLinkRequest>,
) -> Result<(StatusCode, Json<CreateChildLinkResponse>), StatusCode> {
    let child_id = request.child_block_id;

    repos
        .blocks
        .get_by_id(parent_id)
        .await
        .map_err(|e| {
            error!("Failed to get parent block {}: {e}", parent_id);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    repos
        .blocks
        .get_by_id(child_id)
        .await
        .map_err(|e| {
            error!("Failed to get child block {}: {e}", child_id);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    let link_exists = repos
        .block_directional_links
        .exists(parent_id, child_id)
        .await
        .map_err(|e| {
            error!("Failed to check link existence: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if link_exists {
        return Err(StatusCode::CONFLICT);
    }

    let link_create = BlockDirectionalLinkCreate {
        id: Uuid::new_v4(),
        block_from_id: parent_id,
        block_to_id: child_id,
    };

    let link = repos
        .block_directional_links
        .create(link_create)
        .await
        .map_err(|e| match e {
            BlockDirectionalLinkError::CycleDetected { .. } => StatusCode::CONFLICT,
            BlockDirectionalLinkError::SelfLink { .. } => StatusCode::CONFLICT,
            BlockDirectionalLinkError::AlreadyExists { .. } => StatusCode::CONFLICT,
            _ => {
                error!("Failed to create parent link: {e}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;

    let response = CreateChildLinkResponse {
        link_id: link.id,
        parent_block_id: link.block_from_id,
        child_block_id: link.block_to_id,
        created_at: link.created_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}
