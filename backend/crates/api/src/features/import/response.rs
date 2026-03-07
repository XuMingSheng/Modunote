use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ImportResponse {
    pub blocks_inserted: usize,
    pub blocks_updated: usize,
    pub blocks_skipped: usize,
    pub dir_links_inserted: usize,
    pub dir_links_skipped: usize,
    pub related_links_inserted: usize,
    pub related_links_skipped: usize,
}

impl IntoResponse for ImportResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
