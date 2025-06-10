mod delete;
mod load;
mod resolver;
mod save;
mod types;

pub mod macros;

pub use delete::*;
pub use load::*;
pub use resolver::*;
pub use save::*;
pub use types::*;

use crate::data::store::{IndexKey, SortKey};
use thiserror::Error as ThisError;

///
/// ExecutorError
///

#[derive(Debug, ThisError)]
pub enum ExecutorError {
    #[error("key exists: {0}")]
    KeyExists(SortKey),

    #[error("key not found: {0}")]
    KeyNotFound(SortKey),

    #[error("index constraint violation for index: {0:?}")]
    IndexViolation(IndexKey),
}

///
/// DebugContext
///

#[derive(Clone, Debug, Default)]
pub struct DebugContext {
    enabled: bool,
}

impl DebugContext {
    pub const fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn println(&self, s: &str) {
        if self.enabled {
            icu::ic::println!("{s}");
        }
    }
}
