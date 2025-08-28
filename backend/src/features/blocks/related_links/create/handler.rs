use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{error, instrument};
use uuid::Uuid;

use super::{request::CreateRelatedLinkRequest, response::CreateRelatedLinkResponse};
use storage::{
    BlockRelatedLinkCreate, BlockRelatedLinkError, BlockRelatedLinkRepositoryTrait,
    BlockRepositoryTrait, RepositoryProvider,
};

#[utoipa::path(
      post,
      path = "/api/blocks/{id}/related-links",
      tag = "blocks",
      params(
          ("id" = uuid::Uuid, Path, description = "Block ID")
      ),
      request_body = CreateRelatedLinkRequest,
      responses(
          (status = 201, description = "Related link created successfully", body = CreateRelatedLinkResponse),
          (status = 404, description = "Block not found"),
          (status = 409, description = "Link already exists or self-link"),
          (status = 500, description = "Internal server error")
      )
  )]
#[instrument]
pub async fn create_related_link(
    State(repos): State<RepositoryProvider>,
    Path(block_id): Path<Uuid>,
    Json(request): Json<CreateRelatedLinkRequest>,
) -> Result<(StatusCode, Json<CreateRelatedLinkResponse>), StatusCode> {
    let related_block_id = request.related_block_id;

    repos
        .blocks
        .get_by_id(block_id)
        .await
        .map_err(|e| {
            error!("Failed to get block {}: {e}", block_id);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    repos
        .blocks
        .get_by_id(related_block_id)
        .await
        .map_err(|e| {
            error!("Failed to get related block {}: {e}", related_block_id);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    let link_exists = repos
        .block_related_links
        .exists(block_id, related_block_id)
        .await
        .map_err(|e| {
            error!("Failed to check link existence: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if link_exists {
        return Err(StatusCode::CONFLICT);
    }

    let link_create = BlockRelatedLinkCreate {
        id: Uuid::new_v4(),
        block_a_id: block_id,
        block_b_id: related_block_id,
    };

    let link = repos
        .block_related_links
        .create(link_create)
        .await
        .map_err(|e| match e {
            BlockRelatedLinkError::SelfLink { .. } => StatusCode::CONFLICT,
            BlockRelatedLinkError::AlreadyExists { .. } => StatusCode::CONFLICT,
            _ => {
                error!("Failed to create related link: {e}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;

    let response = CreateRelatedLinkResponse {
        link_id: link.id,
        block_a_id: link.block_a_id,
        block_b_id: link.block_b_id,
        created_at: link.created_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}
