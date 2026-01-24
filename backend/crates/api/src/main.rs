use std::sync::Arc;

use api::app_state::AppState;
use axum::Router;
use tracing::instrument;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use api::config::DatabaseImpl;
use api::features;
use api::telemetry::initialize_tracing;
use api::{AppConfig, AppResult as Result};
use storage::database::Database;

#[instrument]
async fn setup_db(database_url: &str) -> Result<DatabaseImpl> {
    let db = DatabaseImpl::connect(database_url).await?;

    db.run_migration().await?;

    tracing::info!("Database initialized and migrated.");

    Ok(db)
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::from_env()?;

    initialize_tracing(&config.log_dir_path)?;

    let db = setup_db(&config.database_url).await?;

    let state = Arc::new(AppState::new(db));

    let (router, openapi) = OpenApiRouter::new()
        .merge(features::blocks::routes())
        .merge(features::block_links::routes())
        .merge(features::workspace::routes())
        .merge(features::search::routes())
        .split_for_parts();

    let app = Router::new()
        .merge(router)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
        .with_state(state);

    let addr = "0.0.0.0:8080";

    let listener = tokio::net::TcpListener::bind(&addr).await?;

    axum::serve(listener, app).await?;

    tracing::info!("Server starting on on http://{addr}");

    Ok(())
}
