pub mod delete;
pub mod load;
pub mod resolver;
pub mod save;
pub mod types;

pub use delete::{DeleteBuilder, DeleteError, DeleteExecutor, DeleteQuery, DeleteResponse};
pub use load::{
    LoadBuilder, LoadBuilderDyn, LoadError, LoadExecutor, LoadExecutorDyn, LoadQuery, LoadQueryDyn,
    LoadResponse, LoadResponseDyn,
};
pub use resolver::{Resolver, ResolverError};
pub use save::{
    SaveBuilder, SaveBuilderDyn, SaveError, SaveMode, SaveQuery, SaveQueryDyn, SaveResponse,
    SaveResponseDyn,
};
pub use types::*;

use crate::{ThisError, orm::traits::Entity};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// QueryError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum QueryError {
    #[error(transparent)]
    DeleteError(#[from] DeleteError),

    #[error(transparent)]
    LoadError(#[from] LoadError),

    #[error(transparent)]
    SaveError(#[from] SaveError),
}

// load
#[must_use]
pub fn load<E>() -> LoadBuilder
where
    E: Entity,
{
    LoadBuilder::new(E::PATH)
}

// load_dyn
#[must_use]
pub fn load_dyn(path: &str) -> LoadBuilderDyn {
    LoadBuilderDyn::new(path)
}

// delete
#[must_use]
pub fn delete<E: Entity>() -> DeleteBuilder {
    DeleteBuilder::new(E::PATH)
}

// save
#[must_use]
pub const fn save(mode: SaveMode) -> SaveBuilder {
    SaveBuilder::new(mode)
}

// save_dyn
#[must_use]
pub fn save_dyn(path: &str, mode: SaveMode) -> SaveBuilderDyn {
    SaveBuilderDyn::new(path, mode)
}

// create
#[must_use]
pub const fn create() -> SaveBuilder {
    SaveBuilder::new(SaveMode::Create)
}

// create_dyn
#[must_use]
pub fn create_dyn(path: &str) -> SaveBuilderDyn {
    SaveBuilderDyn::new(path, SaveMode::Create)
}

// replace
#[must_use]
pub const fn replace() -> SaveBuilder {
    SaveBuilder::new(SaveMode::Replace)
}

// replace_dyn
#[must_use]
pub fn replace_dyn(path: &str) -> SaveBuilderDyn {
    SaveBuilderDyn::new(path, SaveMode::Replace)
}

// update
#[must_use]
pub const fn update() -> SaveBuilder {
    SaveBuilder::new(SaveMode::Update)
}

// update_dyn
#[must_use]
pub fn update_dyn(path: &str) -> SaveBuilderDyn {
    SaveBuilderDyn::new(path, SaveMode::Update)
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
