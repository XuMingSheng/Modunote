use serde::Deserialize;

use crate::{AppError, AppResult as Result};

#[derive(Deserialize, Debug)]
pub struct TelemetryConfig {
    pub level: String,
    pub otel_exporter_otlp_endpoint: String,
}

impl TelemetryConfig {
    pub fn load(table: &toml::Table) -> Result<Self> {
        let sub_table = table.get("telemetry").and_then(|v| v.as_table());

        let config = Self {
            level: load_value("level", "RUST_LOG_LEVEL", sub_table)?,
            otel_exporter_otlp_endpoint: load_value(
                "otel_exporter_otlp_endpoin",
                "OTEL_EXPORTER_OTLP_ENDPOINT",
                sub_table,
            )?,
        };

        Ok(config)
    }
}

fn load_value<T>(key: &str, env_var: &str, table: Option<&toml::Table>) -> Result<T>
where
    T: std::str::FromStr + serde::de::DeserializeOwned,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    if let Ok(env_str) = std::env::var(env_var) {
        let val = env_str.parse::<T>().map_err(|e| AppError::ParseError {
            var_name: format!("Env var: {env_var}"),
            source: Box::new(e),
        })?;

        return Ok(val);
    }

    if let Some(value) = table.and_then(|t| t.get(key)) {
        let val = value
            .clone()
            .try_into::<T>()
            .map_err(|e| AppError::ParseError {
                var_name: format!("TOML key: {key}"),
                source: Box::new(e),
            })?;

        return Ok(val);
    }

    Err(AppError::MissingConfig(format!("{env_var} / {key}",)))
}
