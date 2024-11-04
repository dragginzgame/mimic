use mimic::core::state::app_state::{AppMode, AppStateManager};
use serde::{Deserialize, Serialize};
use snafu::prelude::*;

///
/// Error
///
/// The guard functions just use String, but that's fine they can be set
/// up as Snafu errors
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("app is disabled"))]
    AppDisabled,

    #[snafu(display("app is readonly"))]
    AppReadonly,
}

// guard_query
pub fn guard_query() -> Result<(), String> {
    match AppStateManager::get_mode() {
        AppMode::Enabled | AppMode::Readonly => Ok(()),
        AppMode::Disabled => Err(Error::AppDisabled.to_string()),
    }
}

// guard_update
pub fn guard_update() -> Result<(), String> {
    match AppStateManager::get_mode() {
        AppMode::Enabled => Ok(()),
        AppMode::Readonly => Err(Error::AppReadonly.to_string()),
        AppMode::Disabled => Err(Error::AppDisabled.to_string()),
    }
}
