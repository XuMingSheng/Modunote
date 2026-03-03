#[derive(thiserror::Error, Debug)]
pub enum TelemetryError {
    #[error(transparent)]
    OtlpExpoterBuild(#[from] opentelemetry_otlp::ExporterBuildError),
}

pub type TelemetryResult<T> = Result<T, TelemetryError>;
