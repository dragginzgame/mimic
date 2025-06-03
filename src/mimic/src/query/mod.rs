mod delete;
mod load;
mod save;
mod types;

pub use delete::*;
pub use load::*;
pub use save::*;
pub use types::*;

use crate::{Error, deserialize, traits::Entity};

// load
#[must_use]
pub fn load<E: Entity>() -> LoadQueryBuilder<E> {
    LoadQueryBuilder::<E>::new()
}

// load_dyn
#[must_use]
pub fn load_dyn<E: Entity>() -> LoadQueryDynBuilder<E> {
    LoadQueryDynBuilder::new()
}

// delete
#[must_use]
pub const fn delete() -> DeleteQueryBuilder {
    DeleteQueryBuilder::new()
}

// save
pub fn save<E: Entity + 'static>(query: SaveQuery) -> Result<SaveQueryPrepared, Error> {
    let entity = deserialize::<E>(&query.bytes)?;

    Ok(SaveQueryPrepared::new(query.mode, Box::new(entity)))
}

// create
#[must_use]
pub fn create() -> SaveQueryBuilder {
    SaveQueryBuilder::new(SaveMode::Create)
}

// update
#[must_use]
pub fn update() -> SaveQueryBuilder {
    SaveQueryBuilder::new(SaveMode::Update)
}

// replace
#[must_use]
pub fn replace() -> SaveQueryBuilder {
    SaveQueryBuilder::new(SaveMode::Replace)
}
