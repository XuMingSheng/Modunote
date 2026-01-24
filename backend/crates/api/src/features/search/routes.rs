use std::sync::Arc;

use crate::AppState;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn routes() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new().routes(routes!(super::blocks::search_blocks))
}
