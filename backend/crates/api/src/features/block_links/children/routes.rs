use std::sync::Arc;

use crate::AppState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn routes() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new().routes(routes!(
        super::create::create_block_child_link,
        super::delete::delete_block_child_link,
        super::get::get_block_child_links
    ))
}
