pub mod config;

pub use config::Config;

use candid::CandidType;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::sync::Mutex;

///
/// CONFIG
/// Global static variable
///

static CONFIG: Mutex<Option<Config>> = Mutex::new(None);

// get
pub fn get() -> Result<Config, Error> {
    let config_guard = CONFIG
        .lock()
        .map_err(|e| Error::Mutex { msg: e.to_string() })?;

    match *config_guard {
        Some(ref config) => Ok(config.clone()),
        None => Err(Error::NotInitialized),
    }
}

// init
// private as we don't need to update it directly with a config object
fn init(config: Config) -> Result<(), Error> {
    let mut config_guard = CONFIG
        .lock()
        .map_err(|e| Error::Mutex { msg: e.to_string() })?;

    if config_guard.is_some() {
        Err(Error::AlreadyInitialized)
    } else {
        *config_guard = Some(config);

        Ok(())
    }
}

// init_toml
pub fn init_toml(config_str: &str) -> Result<(), Error> {
    let config =
        toml::from_str(config_str).map_err(|e| Error::CannotParseToml { msg: e.to_string() })?;

    init(config)
}

///
/// Error
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("config has already been initialized"))]
    AlreadyInitialized,

    #[snafu(display("config not yet initialized"))]
    NotInitialized,

    #[snafu(display("mutex error: {msg}"))]
    Mutex { msg: String },

    #[snafu(display("toml error: {msg}"))]
    CannotParseToml { msg: String },
}
