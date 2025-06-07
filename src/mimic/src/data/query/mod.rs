mod delete;
mod load;
mod save;
mod types;

pub use delete::*;
pub use load::*;
pub use save::*;
pub use types::*;

use crate::{Error, deserialize, traits::EntityKind};
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

// save
pub fn save<E: EntityKind + 'static>(query: SaveQuery) -> Result<SaveQueryPrepared, Error> {
    let entity = deserialize::<E>(&query.bytes)?;

    Ok(SaveQueryPrepared::new(query.mode, Box::new(entity)))
}

// create
#[must_use]
pub const fn create() -> SaveQueryBuilder {
    SaveQueryBuilder::new(SaveMode::Create)
}

// update
#[must_use]
pub const fn update() -> SaveQueryBuilder {
    SaveQueryBuilder::new(SaveMode::Update)
}

// replace
#[must_use]
pub const fn replace() -> SaveQueryBuilder {
    SaveQueryBuilder::new(SaveMode::Replace)
}
