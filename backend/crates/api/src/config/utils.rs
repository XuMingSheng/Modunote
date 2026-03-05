use super::error::{ConfigError, ConfigResult as Result};

pub fn load_value<T>(env_var: &str, key: &str, table: &toml::Table) -> Result<T>
where
    T: std::str::FromStr + serde::de::DeserializeOwned,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    if let Ok(env_str) = std::env::var(env_var) {
        let val = env_str.parse::<T>().map_err(|e| ConfigError::ParseError {
            var_name: format!("Env var: {env_var}"),
            source: Box::new(e),
        })?;

        return Ok(val);
    }

    if let Some(value) = table.get(key) {
        let val = value
            .clone()
            .try_into::<T>()
            .map_err(|e| ConfigError::ParseError {
                var_name: format!("TOML key: {key}"),
                source: Box::new(e),
            })?;

        return Ok(val);
    }

    Err(ConfigError::MissingValue(format!("{env_var} / {key}",)))
}
