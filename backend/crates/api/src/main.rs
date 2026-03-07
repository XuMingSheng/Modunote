use std::sync::Arc;

use api::app_state::AppState;
use axum::http::HeaderValue;
use axum::{Router, http::Method, http::header};
use tower_http::cors::CorsLayer;
use tracing::instrument;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use api::app_state::DatabaseImpl;
use api::features;
use api::telemetry::{initialize_tracing, set_panic_hook};
use api::{AppConfig, AppError, AppResult as Result};
use storage::database::Database;

#[instrument]
async fn setup_db(database_url: &str) -> Result<DatabaseImpl> {
    let db = DatabaseImpl::connect(database_url).await?;

    db.run_migration().await?;

    tracing::info!("Database initialized and migrated.");

    Ok(db)
}

fn configure_cors(config: &AppConfig) -> Result<CorsLayer> {
    let frontend_url =
        config
            .frontend_url
            .parse::<HeaderValue>()
            .map_err(|e| AppError::ParseError {
                var_name: "config.frontend_url".to_string(),
                source: Box::new(e),
            })?;

    let cors = CorsLayer::new()
        .allow_origin(frontend_url)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
            Method::OPTIONS,
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
        .allow_credentials(true);

    Ok(cors)
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::load()?;

    initialize_tracing(&config.telemetry)?;
    set_panic_hook();

    let db = setup_db(&config.database_url).await?;

    let state = Arc::new(AppState::new(db));

    let (router, openapi) = OpenApiRouter::new()
        .merge(features::blocks::routes())
        .merge(features::block_links::routes())
        .merge(features::workspace::routes())
        .merge(features::search::routes())
        .merge(features::export::routes())
        .merge(features::import::routes())
        .split_for_parts();

    let cors_layer = configure_cors(&config)?;

    let app = Router::new()
        .merge(router)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
        .with_state(state)
        .layer(cors_layer);

    let addr = "0.0.0.0:8080";

    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Server starting on on http://{addr}");

    axum::serve(listener, app).await?;

    Ok(())
}
