pub mod app_state;
pub mod error;
pub mod features;

use utoipa::OpenApi;

pub use app_state::AppState;
use features::blocks;
use features::search;
use features::workspace;

#[derive(OpenApi)]
#[openapi(
        paths(
            blocks::get::get_block,
            blocks::create::create_block,
            blocks::update::update_block,
            blocks::delete::handler::delete_block,
            blocks::parents::get::get_parent_blocks,
            blocks::parents::create::create_parent_link,
            blocks::parents::delete::delete_parent_link,
            blocks::children::get::get_child_blocks,
            blocks::children::create::create_child_link,
            blocks::children::delete::delete_child_link,
            blocks::related_links::get::get_related_blocks,
            blocks::related_links::create::create_related_link,
            blocks::related_links::delete::delete_related_link,
            workspace::opened_blocks::get::get_opened_blocks,
            workspace::opened_blocks::create::open_block,
            workspace::opened_blocks::delete::close_block,
            search::blocks::search_blocks
        ),
        components(
        ),
        tags(
            (name = "blocks", description = "Block management API"),
            (name = "search", description = "Temporary endpoints for search and list")
        )
  )]
pub struct ApiDoc;
