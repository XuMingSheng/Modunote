use crate::app_state::AppState;
use axum::Router;
use utoipa_axum::{router::OpenApiRouter, routes};

use super::blocks;

pub fn routes() -> Router<AppState> {
    let (router, _) = OpenApiRouter::new()
        .routes(routes!(blocks::search_blocks))
        .split_for_parts();
    router
}
