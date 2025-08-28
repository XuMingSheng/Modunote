use crate::app_state::AppState;
use axum::Router;

use super::opened_blocks;

pub fn routes() -> Router<AppState> {
    Router::new().merge(opened_blocks::routes())
}
