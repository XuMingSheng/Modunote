use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use storage::query_services::block_link_query_service::BlockLinkQueryServiceError;
use storage::query_services::block_query_service::BlockQueryServiceError;
use storage::repositories::block_directional_link_repository::BlockDirectionalLinkRepositoryError;
use storage::repositories::block_related_link_repository::BlockRelatedLinkError;
use storage::repositories::block_repository::BlockRepositoryError;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum ImportError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    BlockQueryService(#[from] BlockQueryServiceError),

    #[error(transparent)]
    BlockLinkQueryService(#[from] BlockLinkQueryServiceError),

    #[error(transparent)]
    BlockRepository(#[from] BlockRepositoryError),

    #[error(transparent)]
    BlockDirectionalLinkRepository(#[from] BlockDirectionalLinkRepositoryError),

    #[error(transparent)]
    BlockRelatedLinkRepository(#[from] BlockRelatedLinkError),

    #[error("Failed to read archive")]
    Zip(#[from] zip::result::ZipError),

    #[error("Failed to deserialize data")]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Multipart(#[from] axum::extract::multipart::MultipartError),

    #[error("No file field found in multipart request")]
    MissingFile,
}

impl IntoResponse for ImportError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            Self::MissingFile => (StatusCode::BAD_REQUEST, self.to_string()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Import failed".to_string(),
            ),
        };

        (status, Json(ErrorResponse { error: msg })).into_response()
    }
}
