use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use tracing::error;
use utoipa::ToSchema;

use storage::query_services::block_query_service::BlockQueryServiceError;
use storage::repositories::workspace_repository::WorkspaceRepositoryError;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ErrorResponse {
    pub error: String,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum OpenBlockError {
    #[error("Validation failed: {0}")]
    InputValidation(String),

    #[error(transparent)]
    WorkspaceRepository(#[from] WorkspaceRepositoryError),

    #[error(transparent)]
    BlockQueryService(#[from] BlockQueryServiceError),

    #[error("Opened block not found after save")]
    OpenedBlockMissing,
}

impl IntoResponse for OpenBlockError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            Self::InputValidation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            Self::WorkspaceRepository(err) => match err {
                WorkspaceRepositoryError::SomeBlocksNotFound => {
                    (StatusCode::NOT_FOUND, "Block not found".to_string())
                }
                other => {
                    error!(error = ?other, "Workspace repository failure");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal server error".to_string(),
                    )
                }
            },
            Self::BlockQueryService(err) => {
                error!(error = ?err, "Query service failure");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            Self::OpenedBlockMissing => {
                error!("Opened block missing after save");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        let body = Json(ErrorResponse { error: msg });

        (status, body).into_response()
    }
}
