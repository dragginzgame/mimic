pub mod delete;
pub mod load;
pub mod resolver;
pub mod save;
pub mod types;

pub use delete::{DeleteBuilder, DeleteError, DeleteExecutor, DeleteQuery, DeleteResponse};
pub use load::{
    LoadBuilder, LoadBuilderDyn, LoadError, LoadExecutor, LoadQuery, LoadResponse, LoadResponseDyn,
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
pub fn load<E: Entity>() -> LoadBuilder<E> {
    LoadBuilder::<E>::new()
}

// load_dyn
#[must_use]
pub fn load_dyn<E: Entity>() -> LoadBuilderDyn<E> {
    LoadBuilderDyn::<E>::new()
}

// delete
#[must_use]
pub fn delete<E: Entity>() -> DeleteBuilder<E> {
    DeleteBuilder::<E>::new()
}

// save
#[must_use]
pub fn save<E: Entity>(mode: SaveMode) -> SaveBuilder {
    SaveBuilder::new(E::PATH, mode)
}

// save_dyn
#[must_use]
pub fn save_dyn(mode: SaveMode) -> SaveBuilderDyn {
    SaveBuilderDyn::new(mode)
}

// create
#[must_use]
pub fn create<E: Entity>() -> SaveBuilder {
    SaveBuilder::new(E::PATH, SaveMode::Create)
}

// create_dyn
#[must_use]
pub fn create_dyn() -> SaveBuilderDyn {
    SaveBuilderDyn::new(SaveMode::Create)
}

// replace
#[must_use]
pub fn replace<E: Entity>() -> SaveBuilder {
    SaveBuilder::new(E::PATH, SaveMode::Replace)
}

// replace_dyn
#[must_use]
pub fn replace_dyn() -> SaveBuilderDyn {
    SaveBuilderDyn::new(SaveMode::Replace)
}

// update
#[must_use]
pub fn update<E: Entity>() -> SaveBuilder {
    SaveBuilder::new(E::PATH, SaveMode::Update)
}

// update_dyn
#[must_use]
pub fn update_dyn() -> SaveBuilderDyn {
    SaveBuilderDyn::new(SaveMode::Update)
}

///
/// DebugContext
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
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
