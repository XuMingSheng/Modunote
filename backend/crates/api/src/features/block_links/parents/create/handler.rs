use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{error, instrument};
use utoipa;
use uuid::Uuid;

use super::{request::CreateBlockParentLinkRequest, response::CreateBlockParentLinkResponse};
use crate::AppState;
use crate::features::error::{ErrorResponse, HandlerError};
use storage::{
    repositories::BlockDirectionalLinkRepository,
    repositories::block_directional_link_repository::BlockDirectionalLinkRepositoryError,
    repositories::block_directional_link_repository::CreateBlockDirectionalLinkDto,
};

#[instrument]
#[utoipa::path(
    post,
    path = "/api/blocks/{id}/parents",
    tag = "block_links",
    responses(
        (status = StatusCode::CREATED, description = "Block link created successfully", body = CreateBlockParentLinkResponse),
        (status = StatusCode::BAD_REQUEST, description = "Validation error", body = ErrorResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = ErrorResponse),
    )
)]
pub async fn create_block_parent_link(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(request): Json<CreateBlockParentLinkRequest>,
) -> Result<(StatusCode, Json<CreateBlockParentLinkResponse>), HandlerError> {
    let link_id = Uuid::new_v4();
    let block_from_id = request.parent_block_id;

    let input = CreateBlockDirectionalLinkDto {
        id: link_id,
        block_from_id,
        block_to_id: id,
    };

    let link = state
        .repos
        .block_directional_links
        .create(&input)
        .await
        .map_err(|e| match e {
            BlockDirectionalLinkRepositoryError::BlocksNotFound { .. }
            | BlockDirectionalLinkRepositoryError::CycleDetected { .. }
            | BlockDirectionalLinkRepositoryError::AlreadyExists { .. } => {
                HandlerError::ValidationError(e.to_string())
            }
            _ => {
                error!("Failed to create block directional link {block_from_id} -> {id}: {e}");
                HandlerError::Anyhow
            }
        })?;

    let response: CreateBlockParentLinkResponse = link.into();

    Ok((StatusCode::CREATED, Json(response)))
}
