use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use tracing::instrument;
use utoipa;
use uuid::Uuid;

use super::{
    error::{CreateBlockRelatedLinkError, ErrorResponse},
    request::CreateBlockRelatedLinkRequest,
    response::CreateBlockRelatedLinkResponse,
};
use crate::AppState;
use storage::{
    Database, repositories::BlockRelatedLinkRepository,
    repositories::block_related_link_repository::CreateBlockRelatedLinkDto,
};

#[instrument]
#[utoipa::path(
    post,
    path = "/api/blocks/{id}/related",
    tag = "block_links",
    responses(
        (status = 201, description = "Block link created successfully", body = CreateBlockRelatedLinkResponse),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    )
)]
pub async fn create_block_related_link(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(request): Json<CreateBlockRelatedLinkRequest>,
) -> Result<CreateBlockRelatedLinkResponse, CreateBlockRelatedLinkError> {
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
        .create(&input, state.db.pool())
        .await?;

    let response: CreateBlockRelatedLinkResponse = link.into();

    Ok(response)
}
