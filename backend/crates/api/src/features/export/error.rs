use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use storage::query_services::block_link_query_service::BlockLinkQueryServiceError;
use storage::query_services::block_query_service::BlockQueryServiceError;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum ExportError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Http(#[from] axum::http::Error),

    #[error(transparent)]
    BlockQueryService(#[from] BlockQueryServiceError),

    #[error(transparent)]
    BlockLinkQueryService(#[from] BlockLinkQueryServiceError),

    #[error("Failed to build export archive")]
    Zip(#[from] zip::result::ZipError),

    #[error("Failed to serialize data")]
    Serialization(#[from] serde_json::Error),
}

impl IntoResponse for ExportError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Export failed".to_string(),
            }),
        )
            .into_response()
    }
}
