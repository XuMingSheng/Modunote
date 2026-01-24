use std::sync::Arc;

use crate::AppState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn routes() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new().routes(routes!(
        super::get::get_opened_blocks,
        super::create::open_block,
        super::delete::close_block
    ))
}
