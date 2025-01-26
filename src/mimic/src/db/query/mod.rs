pub mod delete;
pub mod load;
pub mod resolver;
pub mod save;
pub mod types;

pub use delete::{DeleteBuilder, DeleteExecutor, DeleteQuery, DeleteResponse, EDeleteBuilder};
pub use load::{
    ELoadBuilder, ELoadExecutor, ELoadQuery, ELoadResult, LoadBuilder, LoadExecutor, LoadQuery,
    LoadResult,
};
pub use resolver::Resolver;
pub use save::{ESaveBuilder, SaveBuilder, SaveMode};
pub use types::*;

use crate::orm::traits::Entity;
use candid::CandidType;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// QueryError
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum QueryError {
    #[snafu(transparent)]
    Delete { source: delete::DeleteError },

    #[snafu(transparent)]
    Load { source: load::LoadError },

    #[snafu(transparent)]
    Save { source: save::SaveError },

    #[snafu(transparent)]
    Orm { source: crate::orm::OrmError },
}

///
/// Query Builders
///

// load_entity
#[must_use]
pub fn load_entity<E>() -> ELoadBuilder<E>
where
    E: Entity + 'static,
{
    ELoadBuilder::<E>::new()
}

// load
#[must_use]
pub fn load(path: &str) -> LoadBuilder {
    LoadBuilder::new(path)
}

// delete
#[must_use]
pub fn delete(path: &str) -> DeleteBuilder {
    DeleteBuilder::new(path)
}

// delete_entity
#[must_use]
pub fn delete_entity<E>() -> EDeleteBuilder<E>
where
    E: Entity + 'static,
{
    EDeleteBuilder::<E>::new()
}

// create
#[must_use]
pub fn create() -> SaveBuilder {
    SaveBuilder::new(SaveMode::Create)
}

// create_entity
#[must_use]
pub fn create_entity<E>() -> ESaveBuilder<E>
where
    E: Entity + 'static,
{
    ESaveBuilder::<E>::new(SaveMode::Create)
}

// replace
#[must_use]
pub fn replace() -> SaveBuilder {
    SaveBuilder::new(SaveMode::Replace)
}

// replace_entity
#[must_use]
pub fn replace_entity<E>() -> ESaveBuilder<E>
where
    E: Entity + 'static,
{
    ESaveBuilder::<E>::new(SaveMode::Replace)
}

// update
#[must_use]
pub fn update() -> SaveBuilder {
    SaveBuilder::new(SaveMode::Update)
}

// update_entity
#[must_use]
pub fn update_entity<E>() -> ESaveBuilder<E>
where
    E: Entity + 'static,
{
    ESaveBuilder::<E>::new(SaveMode::Update)
}

///
/// DebugContext
///

#[derive(CandidType, Debug, Default, Serialize, Deserialize)]
pub struct DebugContext {
    enabled: bool,
}

impl DebugContext {
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn println(&self, s: &str) {
        if self.enabled {
            crate::ic::println!("{s}");
        }
    }
}
