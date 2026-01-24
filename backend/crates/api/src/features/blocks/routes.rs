use std::sync::Arc;

use crate::AppState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn routes() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new().routes(routes!(
        super::get::get_block,
        super::create::create_block,
        super::update::update_block,
        super::delete::delete_block
    ))
}
