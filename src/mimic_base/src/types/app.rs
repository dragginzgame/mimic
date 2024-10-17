use crate::{sanitizer, types, validator};
use mimic::orm::prelude::*;

///
/// Version
///
/// takes the currrent app::Version from config
///

#[newtype(
    primitive = "String",
    value(
        item(is = "types::String"),
        default = "types::app::Version::app_version"
    )
)]
pub struct Version {}

impl Version {
    #[must_use]
    pub fn app_version() -> String {
        if let Ok(config) = ::core_config::get_config() {
            config.app.version.clone()
        } else {
            "[no config]".to_string()
        }
    }
}
