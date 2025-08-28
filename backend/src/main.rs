use axum::{Router, routing::get};
use std::env;
use tracing::instrument;
use tracing_appender::non_blocking;
use tracing_appender::rolling::RollingFileAppender;
use tracing_subscriber::prelude::*;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use backend::ApiDoc;
use backend::AppState;
use backend::error::Result;
use backend::features;
use storage::Database;
use storage::RepositoryProvider;

async fn ping() -> &'static str {
    "pong"
}

fn initialize_tracing() {
    let file_appender = RollingFileAppender::new(
        tracing_appender::rolling::Rotation::DAILY,
        "./logs",
        "backend_log",
    );
    let (non_blocking_appender, _guard) = non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer()) // Console Output
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking_appender) // File Output
                .with_ansi(false),
        )
        .init();
}

#[instrument]
async fn initialize_database_and_repositories() -> Result<(Database, RepositoryProvider)> {
    let database_url = env::var("DATABASE_URL")?;

    let db = Database::new(&database_url).await?;
    let repos = RepositoryProvider::new(db.pool());

    tracing::info!("Database and repositories initialized.");

    Ok((db, repos))
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    dotenvy::from_path("backend/.env").ok();

    initialize_tracing();

    let (db, repos) = initialize_database_and_repositories().await?;

    let state = AppState { db, repos };

    let app = Router::new()
        .route("/api/ping", get(ping))
        .merge(features::blocks::routes())
        .merge(features::workspace::routes())
        .merge(features::search::routes())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state);

    let addr = "0.0.0.0:8080";

    let listener = tokio::net::TcpListener::bind(&addr).await?;

    axum::serve(listener, app).await?;

    tracing::info!("Server starting on on http://{addr}");

    Ok(())
}
