use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use tracing::error;
use utoipa::ToSchema;

use storage::query_services::block_link_query_service::BlockLinkQueryServiceError;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ErrorResponse {
    pub error: String,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum GetBlockParentLinksError {
    #[error(transparent)]
    BlockLinkQueryService(#[from] BlockLinkQueryServiceError),
}

impl IntoResponse for GetBlockParentLinksError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            Self::BlockLinkQueryService(err) => {
                error!(error = ?err, "Block link query failure");
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
