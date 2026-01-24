use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use tracing::{error, instrument};
use utoipa;

use super::{request::CreateBlockRequest, response::CreateBlockResponse};
use crate::AppState;
use crate::features::error::HandlerError;
use storage::repositories::BlockRepository;
use storage::repositories::block_repository::CreateBlockDto;

#[instrument]
#[utoipa::path(
    post,
    path = "/api/blocks",
    tag = "blocks",
    responses(
        (status = StatusCode::CREATED, description = "Block created successfully", body = CreateBlockResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
pub async fn create_block(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateBlockRequest>,
) -> Result<(StatusCode, Json<CreateBlockResponse>), HandlerError> {
    let title = request.title.clone();
    let input: CreateBlockDto = request.into();

    let block = state.repos.blocks.create(&input).await.map_err(|e| {
        error!("Failed to create block {title}: {e}");
        HandlerError::Anyhow
    })?;

    let response: CreateBlockResponse = block.into();

    Ok((StatusCode::CREATED, Json(response)))
}
