mod delete;
mod filter;
mod load;
mod planner;
mod range;
mod save;
mod sort;

pub use delete::*;
pub use filter::*;
pub use load::*;
pub use planner::*;
pub use range::*;
pub use save::*;
pub use sort::*;

use thiserror::Error as ThisError;

///
/// QueryError
///

#[derive(Debug, ThisError)]
pub enum QueryError {
    #[error("selector not supported")]
    SelectorNotSupported,
}

// load
#[must_use]
pub fn load() -> LoadQuery {
    LoadQuery::new()
}

// delete
#[must_use]
pub fn delete() -> DeleteQuery {
    DeleteQuery::new()
}

// create
#[must_use]
pub const fn create() -> SaveQueryTypedBuilder {
    SaveQueryTypedBuilder::new(SaveMode::Create)
}

// update
#[must_use]
pub const fn update() -> SaveQueryTypedBuilder {
    SaveQueryTypedBuilder::new(SaveMode::Update)
}

// replace
#[must_use]
pub const fn replace() -> SaveQueryTypedBuilder {
    SaveQueryTypedBuilder::new(SaveMode::Replace)
}
