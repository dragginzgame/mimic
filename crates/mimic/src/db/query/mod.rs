mod delete;
mod load;
mod save;

pub use delete::*;
pub use load::*;
pub use save::*;

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

// load_dyn
#[must_use]
pub fn load_dyn() -> LoadQueryDynBuilder {
    LoadQueryDynBuilder::new()
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
