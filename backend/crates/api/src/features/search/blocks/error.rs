use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use tracing::error;
use utoipa::ToSchema;

use storage::query_services::block_query_service::BlockQueryServiceError;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ErrorResponse {
    pub error: String,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum SearchBlocksError {
    #[error("Validation failed: {0}")]
    InputValidation(String),

    #[error(transparent)]
    BlockQueryService(#[from] BlockQueryServiceError),
}

impl IntoResponse for SearchBlocksError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            Self::InputValidation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            Self::BlockQueryService(err) => {
                error!(error = ?err, "Query service failure");
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
