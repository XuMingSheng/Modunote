use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ErrorResponse {
    pub error: String,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum HandlerError {
    #[error("Resource not found")]
    NotFound,

    #[error("Validation failed: {0}")]
    ValidationError(String),

    #[error("Internal server error")]
    Anyhow,
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        let status = match self {
            HandlerError::NotFound => StatusCode::NOT_FOUND,
            HandlerError::ValidationError(_) => StatusCode::BAD_REQUEST,
            HandlerError::Anyhow => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(ErrorResponse {
            error: self.to_string(),
        });

        (status, body).into_response()
    }
}
