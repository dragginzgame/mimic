pub mod delete;
pub mod load;
pub mod resolver;
pub mod save;
pub mod types;

pub use delete::{
    DeleteBuilder, DeleteBuilderDyn, DeleteError, DeleteExecutor, DeleteExecutorDyn, DeleteQuery,
    DeleteQueryDyn, DeleteResponse,
};
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
pub const fn load<E: Entity>() -> LoadBuilder<E> {
    LoadBuilder::new()
}

// load_dyn
#[must_use]
pub fn load_dyn(path: &str) -> LoadBuilderDyn {
    LoadBuilderDyn::new(path)
}

// delete
#[must_use]
pub const fn delete<E: Entity>() -> DeleteBuilder<E> {
    DeleteBuilder::new()
}

// delete_dyn
#[must_use]
pub fn delete_dyn(path: &str) -> DeleteBuilderDyn {
    DeleteBuilderDyn::new(path)
}

// create
#[must_use]
pub const fn create<E: Entity>() -> SaveBuilder<E> {
    SaveBuilder::new(SaveMode::Create)
}

// create_dyn
#[must_use]
pub const fn create_dyn() -> SaveBuilderDyn {
    SaveBuilderDyn::new(SaveMode::Create)
}

// replace
#[must_use]
pub const fn replace<E: Entity>() -> SaveBuilder<E> {
    SaveBuilder::new(SaveMode::Replace)
}

// replace_dyn
#[must_use]
pub const fn replace_dyn() -> SaveBuilderDyn {
    SaveBuilderDyn::new(SaveMode::Replace)
}

// update
#[must_use]
pub const fn update<E: Entity>() -> SaveBuilder<E> {
    SaveBuilder::new(SaveMode::Update)
}

// update_dyn
#[must_use]
pub const fn update_dyn() -> SaveBuilderDyn {
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
