pub mod types;

pub use types::Config;

use candid::CandidType;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::sync::Mutex;

///
/// ConfigError
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum ConfigError {
    #[snafu(display("config has already been initialized"))]
    AlreadyInitialized,

    #[snafu(display("config not yet initialized"))]
    NotInitialized,

    #[snafu(display("mutex error: {msg}"))]
    Mutex { msg: String },

    #[snafu(display("toml error: {msg}"))]
    CannotParseToml { msg: String },
}

///
/// CONFIG
/// Global static variable
///

static CONFIG: Mutex<Option<Config>> = Mutex::new(None);

// get_config
pub fn get_config() -> Result<Config, ConfigError> {
    let guard = CONFIG
        .lock()
        .map_err(|e| ConfigError::Mutex { msg: e.to_string() })?;

    guard.as_ref().map_or(
        Err(ConfigError::NotInitialized),
        |config| Ok(config.clone()),
    )
}

// init_config
fn init_config(config: Config) -> Result<(), ConfigError> {
    let mut guard = CONFIG
        .lock()
        .map_err(|e| ConfigError::Mutex { msg: e.to_string() })?;

    if guard.is_some() {
        Err(ConfigError::AlreadyInitialized)
    } else {
        *guard = Some(config);

        Ok(())
    }
}

// init_config_toml
pub fn init_config_toml(config_str: &str) -> Result<(), ConfigError> {
    let config = toml::from_str(config_str)
        .map_err(|e| ConfigError::CannotParseToml { msg: e.to_string() })?;

    init_config(config)
}
