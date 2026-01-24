use tracing_appender::non_blocking;
use tracing_appender::rolling::RollingFileAppender;
use tracing_subscriber::prelude::*;

use super::AppResult as Result;

pub fn initialize_tracing(log_dir_path: &str) -> Result<()> {
    let file_appender = RollingFileAppender::new(
        tracing_appender::rolling::Rotation::DAILY,
        log_dir_path,
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

    Ok(())
}
