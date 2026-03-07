use axum::{
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};

use super::error::ExportError;

pub(crate) struct ExportResponse(Response);

impl ExportResponse {
    pub(crate) fn new(bytes: Vec<u8>) -> Result<Self, ExportError> {
        let response = Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/zip")
            .header(
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"modunote-export.zip\"",
            )
            .body(axum::body::Body::from(bytes))?;

        Ok(Self(response))
    }
}

impl IntoResponse for ExportResponse {
    fn into_response(self) -> Response {
        self.0
    }
}
