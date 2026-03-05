use serde::Deserialize;

use crate::config::{ConfigError, ConfigResult as Result, utils::load_value};

#[derive(Deserialize, Debug)]
pub struct TelemetryConfig {
    pub level: String,
    pub otel_enabled: bool,
    pub otel: Option<OpentelemetryConfig>,
}

#[derive(Deserialize, Debug)]
pub struct OpentelemetryConfig {
    pub service_name: String,
    pub exporter_otlp_endpoint: String,
}

impl TelemetryConfig {
    pub fn load(table: &toml::Table) -> Result<Self> {
        let sub_table = table
            .get("telemetry")
            .and_then(|v| v.as_table())
            .ok_or_else(|| ConfigError::MissingSection("telemetry".to_string()))?;

        let mut config = Self {
            level: load_value("RUST_LOG_LEVEL", "level", sub_table)?,
            otel_enabled: load_value("OTEL_ENABLED", "otel_enabled", sub_table)?,
            otel: None,
        };

        if config.otel_enabled {
            config.otel = Some(OpentelemetryConfig::load(sub_table)?);
        }

        Ok(config)
    }
}

impl OpentelemetryConfig {
    pub fn load(sub_table: &toml::Table) -> Result<Self> {
        let config = Self {
            service_name: load_value("OTEL_SERVICE_NAME", "otel_service_name", sub_table)?,
            exporter_otlp_endpoint: load_value(
                "OTEL_EXPORTER_OTLP_ENDPOINT",
                "otel_exporter_otlp_endpoint",
                sub_table,
            )?,
        };

        Ok(config)
    }
}
