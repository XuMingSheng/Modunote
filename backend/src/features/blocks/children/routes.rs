use utoipa_axum::{router::OpenApiRouter, routes};

use super::{create, delete, get};
use crate::app_state::AppState;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(
        get::get_child_blocks,
        create::create_child_link,
        delete::delete_child_link
    ))
}
