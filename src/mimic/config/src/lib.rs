pub mod config;

pub use config::Config;

use candid::CandidType;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::sync::LazyLock;

///
/// CONFIG
/// Global static variable
///

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    //  let config_str = include_str!("../../../config.toml");
    let config_str = "";

    let config: Config = toml::from_str(config_str)
        .map_err(|_| Error::CannotParseToml)
        .unwrap();

    config
});

///
/// Error
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("cannot parse config.toml"))]
    CannotParseToml,
}
