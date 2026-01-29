use std::sync::Arc;

use crate::AppState;
use utoipa_axum::router::OpenApiRouter;

pub fn routes() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new()
        .merge(super::parents::routes())
        .merge(super::children::routes())
        .merge(super::related::routes())
}
