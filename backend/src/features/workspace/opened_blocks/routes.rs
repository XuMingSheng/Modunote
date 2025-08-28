use crate::app_state::AppState;
use axum::Router;
use utoipa_axum::{router::OpenApiRouter, routes};

use super::create;
use super::delete;
use super::get;

pub fn routes() -> Router<AppState> {
    let (router, _) = OpenApiRouter::new()
        .routes(routes!(
            get::get_opened_blocks,
            create::open_block,
            delete::close_block
        ))
        .split_for_parts();
    router
}
