mod delete;
mod filter;
mod load;
mod planner;
mod save;
mod selector;

pub use delete::*;
pub use filter::*;
pub use load::*;
pub use planner::*;
pub use save::*;
pub use selector::*;

use thiserror::Error as ThisError;

///
/// QueryError
///

#[derive(Debug, ThisError)]
pub enum QueryError {
    #[error("selector not suppoorted")]
    SelectorNotSupported,
}

// load
#[must_use]
pub fn load() -> LoadQueryBuilder {
    LoadQueryBuilder::new()
}

// delete
#[must_use]
pub fn delete() -> DeleteQueryBuilder {
    DeleteQueryBuilder::new()
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
