pub mod build;
pub mod node;
pub mod types;
pub mod visit;

use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(transparent)]
    Build { source: build::Error },
}
