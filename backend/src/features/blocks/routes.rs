use crate::app_state::AppState;
use utoipa_axum::{router::OpenApiRouter, routes};

use super::children;
use super::create;
use super::delete;
use super::get;
use super::parents;
use super::update;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(
            get::get_block,
            create::create_block,
            update::update_block,
            delete::delete_block
        ))
        .merge(parents::routes())
        .merge(children::routes())
}
