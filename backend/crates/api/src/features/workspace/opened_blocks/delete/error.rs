use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use tracing::error;
use utoipa::ToSchema;

use storage::repositories::workspace_repository::WorkspaceRepositoryError;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ErrorResponse {
    pub error: String,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum CloseBlockError {
    #[error("Block is not opened")]
    NotOpened,

    #[error(transparent)]
    WorkspaceRepository(#[from] WorkspaceRepositoryError),
}

impl IntoResponse for CloseBlockError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            Self::NotOpened => (StatusCode::NOT_FOUND, self.to_string()),
            Self::WorkspaceRepository(err) => {
                error!(error = ?err, "Workspace repository failure");
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
