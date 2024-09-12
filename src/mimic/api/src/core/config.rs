use core_config::Config;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(transparent)]
    CoreConfig { source: core_config::Error },
}

// get_config
pub fn get_config() -> Result<Config, Error> {
    core_config::get_config().map_err(Error::from)
}

// init_config_toml
pub fn init_config_toml(config_str: &str) -> Result<(), Error> {
    core_config::init_config_toml(config_str).map_err(Error::from)
}
