use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use tracing::error;
use utoipa::ToSchema;

use storage::repositories::block_directional_link_repository::BlockDirectionalLinkRepositoryError;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ErrorResponse {
    pub error: String,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum CreateBlockChildLinkError {
    #[error("Validation failed: {0}")]
    InputValidation(String),

    #[error(transparent)]
    Repository(#[from] BlockDirectionalLinkRepositoryError),
}

impl IntoResponse for CreateBlockChildLinkError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            Self::InputValidation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            Self::Repository(err) => match err {
                BlockDirectionalLinkRepositoryError::BlocksNotFound { .. }
                | BlockDirectionalLinkRepositoryError::CycleDetected { .. }
                | BlockDirectionalLinkRepositoryError::AlreadyExists { .. } => {
                    (StatusCode::BAD_REQUEST, err.to_string())
                }
                other => {
                    error!(error = ?other, "Directional link repository failure");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal server error".to_string(),
                    )
                }
            },
        };

        let body = Json(ErrorResponse { error: msg });

        (status, body).into_response()
    }
}
