use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use tracing::error;
use utoipa::ToSchema;

use storage::repositories::block_repository::BlockRepositoryError;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ErrorResponse {
    pub error: String,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum UpdateBlockError {
    #[error("Block not found")]
    NotFound,

    #[error(transparent)]
    BlockRepository(#[from] BlockRepositoryError),
}

impl IntoResponse for UpdateBlockError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            Self::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            Self::BlockRepository(err) => {
                error!(error = ?err, "Block repository failure");
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
