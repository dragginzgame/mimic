use serde::{Deserialize, Serialize};
use snafu::Snafu;

// re-export
pub use core_state::*;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(transparent)]
    AppState {
        source: core_state::app_state::Error,
    },
}
