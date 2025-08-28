use utoipa_axum::{router::OpenApiRouter, routes};

use super::{create, delete, get};
use crate::app_state::AppState;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(
        get::get_parent_blocks,
        create::create_parent_link,
        delete::delete_parent_link
    ))
}
