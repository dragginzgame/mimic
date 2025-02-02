pub mod types;

pub use types::Config;

use crate::{Error, ThisError};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

///
/// ConfigError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum ConfigError {
    #[error("config has already been initialized")]
    AlreadyInitialized,

    #[error("toml error: {0}")]
    CannotParseToml(String),

    #[error("config not yet initialized")]
    NotInitialized,

    #[error("mutex error: {0}")]
    Mutex(String),
}

///
/// CONFIG
/// Global static variable
///

static CONFIG: Mutex<Option<Config>> = Mutex::new(None);

// get_config
pub fn get_config() -> Result<Config, Error> {
    let guard = CONFIG
        .lock()
        .map_err(|e| ConfigError::Mutex(e.to_string()))?;

    let config = guard.as_ref().ok_or(ConfigError::NotInitialized).cloned()?;

    Ok(config)
}

// init_config
fn init_config(config: Config) -> Result<(), Error> {
    let mut guard = CONFIG
        .lock()
        .map_err(|e| ConfigError::Mutex(e.to_string()))?;

    if guard.is_some() {
        Err(ConfigError::AlreadyInitialized)?;
    }

    *guard = Some(config);

    Ok(())
}

// init_config_toml
pub fn init_config_toml(config_str: &str) -> Result<(), Error> {
    let config =
        toml::from_str(config_str).map_err(|e| ConfigError::CannotParseToml(e.to_string()))?;

    init_config(config)
}
