use opentelemetry::{KeyValue, trace::TracerProvider};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{Resource, trace::SdkTracerProvider};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{EnvFilter, Layer, Registry, fmt, prelude::*};

use super::error::TelemetryResult as Result;
use crate::telemetry::TelemetryConfig;

pub fn initialize_tracing(config: &TelemetryConfig) -> Result<()> {
    let stdout_layer = fmt::layer()
        .json()
        .with_timer(fmt::time::ChronoUtc::rfc_3339())
        .flatten_event(true)
        .with_current_span(true)
        .with_target(true)
        .with_filter(EnvFilter::new(&config.level));

    let otel_layer = get_otel_layer(config)?;

    tracing_subscriber::registry()
        .with(otel_layer)
        .with(stdout_layer)
        .init();

    Ok(())
}

fn get_otel_layer(
    config: &TelemetryConfig,
) -> Result<OpenTelemetryLayer<Registry, opentelemetry_sdk::trace::Tracer>> {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic() // This requires the "grpc-tonic" feature
        .with_endpoint(&config.otel_exporter_otlp_endpoint)
        .build()?;

    let resource = Resource::builder()
        .with_attributes(vec![
            KeyValue::new("service.name", "modunote.api"),
            KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
        ])
        .build();

    let tracer_provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(resource)
        .build();

    let tracer = tracer_provider.tracer("modunote-api-tracer");
    let layer = tracing_opentelemetry::layer().with_tracer(tracer);

    Ok(layer)
}
