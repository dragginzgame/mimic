pub mod delete;
pub mod load;
pub mod resolver;
pub mod save;
pub mod types;

pub use delete::{
    DeleteBuilder, DeleteBuilderDyn, DeleteError, DeleteExecutor, DeleteQuery, DeleteResponse,
};
pub use load::{
    LoadBuilder, LoadBuilderDyn, LoadError, LoadExecutor, LoadExecutorDyn, LoadQuery, LoadQueryDyn,
    LoadResult, LoadResultDyn,
};
pub use resolver::Resolver;
pub use save::{SaveBuilder, SaveBuilderDyn, SaveError, SaveMode};
pub use types::*;

use crate::orm::traits::Entity;
use candid::CandidType;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// QueryError
/// not a wrapper, just handles any errors that
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum QueryError {
    #[snafu(transparent)]
    DeleteError { source: delete::DeleteError },

    #[snafu(transparent)]
    LoadError { source: load::LoadError },

    #[snafu(transparent)]
    SaveError { source: save::SaveError },
}

///
/// Query Builders
///

// load
#[must_use]
pub fn load<E>() -> LoadBuilder<E>
where
    E: Entity,
{
    LoadBuilder::<E>::new()
}

// load_dyn
#[must_use]
pub fn load_dyn(path: &str) -> LoadBuilderDyn {
    LoadBuilderDyn::new(path)
}

// delete
#[must_use]
pub fn delete<E>() -> DeleteBuilder<E>
where
    E: Entity,
{
    DeleteBuilder::<E>::new()
}

// delete_dyn
#[must_use]
pub fn delete_dyn(path: &str) -> DeleteBuilderDyn {
    DeleteBuilderDyn::new(path)
}

// create
#[must_use]
pub fn create<E>() -> SaveBuilder<E>
where
    E: Entity,
{
    SaveBuilder::<E>::new(SaveMode::Create)
}

// create_dyn
#[must_use]
pub fn create_dyn() -> SaveBuilderDyn {
    SaveBuilderDyn::new(SaveMode::Create)
}

// replace
#[must_use]
pub fn replace<E>() -> SaveBuilder<E>
where
    E: Entity,
{
    SaveBuilder::<E>::new(SaveMode::Replace)
}

// replace_dyn
#[must_use]
pub fn replace_dyn() -> SaveBuilderDyn {
    SaveBuilderDyn::new(SaveMode::Replace)
}

// update
#[must_use]
pub fn update<E>() -> SaveBuilder<E>
where
    E: Entity,
{
    SaveBuilder::<E>::new(SaveMode::Update)
}

// update_dyn
#[must_use]
pub fn update_dyn() -> SaveBuilderDyn {
    SaveBuilderDyn::new(SaveMode::Update)
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
