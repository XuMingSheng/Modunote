use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use tracing::instrument;
use utoipa;
use uuid::Uuid;

use super::{
    error::{CreateBlockChildLinkError, ErrorResponse},
    request::CreateBlockChildLinkRequest,
    response::CreateBlockChildLinkResponse,
};
use crate::AppState;
use storage::{
    Database, repositories::BlockDirectionalLinkRepository,
    repositories::block_directional_link_repository::CreateBlockDirectionalLinkDto,
};

#[instrument]
#[utoipa::path(
    post,
    path = "/api/blocks/{id}/children",
    tag = "block_links",
    responses(
        (status = 201, description = "Block link created successfully", body = CreateBlockChildLinkResponse),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    )
)]
pub async fn create_block_child_link(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(request): Json<CreateBlockChildLinkRequest>,
) -> Result<CreateBlockChildLinkResponse, CreateBlockChildLinkError> {
    let link_id = Uuid::new_v4();
    let block_to_id = request.child_block_id;

    let input = CreateBlockDirectionalLinkDto {
        id: link_id,
        block_from_id: id,
        block_to_id,
    };

    let link = state
        .repos
        .block_directional_links
        .create(&input, state.db.pool())
        .await?;

    let response: CreateBlockChildLinkResponse = link.into();

    Ok(response)
}
