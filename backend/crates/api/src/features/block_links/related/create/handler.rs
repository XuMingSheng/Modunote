use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{error, instrument};
use utoipa;
use uuid::Uuid;

use super::{request::CreateBlockRelatedLinkRequest, response::CreateBlockRelatedLinkResponse};
use crate::AppState;
use crate::features::error::{ErrorResponse, HandlerError};
use storage::{
    repositories::BlockRelatedLinkRepository,
    repositories::block_related_link_repository::BlockRelatedLinkError,
    repositories::block_related_link_repository::CreateBlockRelatedLinkDto,
};

#[instrument]
#[utoipa::path(
    post,
    path = "/api/blocks/{id}/related",
    tag = "block_links",
    responses(
        (status = StatusCode::CREATED, description = "Block link created successfully", body = CreateBlockRelatedLinkResponse),
        (status = StatusCode::BAD_REQUEST, description = "Validation error", body = ErrorResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = ErrorResponse),
    )
)]
pub async fn create_block_related_link(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(request): Json<CreateBlockRelatedLinkRequest>,
) -> Result<(StatusCode, Json<CreateBlockRelatedLinkResponse>), HandlerError> {
    let link_id = Uuid::new_v4();
    let related_id = request.related_block_id;

    let input = CreateBlockRelatedLinkDto {
        id: link_id,
        block_a_id: id,
        block_b_id: related_id,
    };

    let link = state
        .repos
        .block_related_links
        .create(&input)
        .await
        .map_err(|e| match e {
            BlockRelatedLinkError::BlocksNotFound { .. }
            | BlockRelatedLinkError::AlreadyExists { .. }
            | BlockRelatedLinkError::SelfLink { .. } => {
                HandlerError::ValidationError(e.to_string())
            }
            _ => {
                error!("Failed to create related link for blocks {id} <-> {related_id}: {e}");
                HandlerError::Anyhow
            }
        })?;

    let response: CreateBlockRelatedLinkResponse = link.into();

    Ok((StatusCode::CREATED, Json(response)))
}
