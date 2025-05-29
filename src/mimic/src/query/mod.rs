pub mod delete;
pub mod load;
pub mod resolver;
pub mod save;
pub mod traits;
pub mod types;

pub use delete::{DeleteBuilder, DeleteExecutor, DeleteQuery, DeleteResponse};
pub use load::{
    LoadError, LoadMap, LoadQuery, LoadQueryBuilder, LoadQueryDynBuilder, LoadQueryDynExecutor,
    LoadQueryDynInit, LoadQueryInit, LoadResponse,
};
pub use resolver::{Resolver, ResolverError};
pub use save::{
    SaveBuilder, SaveBuilderDyn, SaveError, SaveExecutor, SaveExecutorDyn, SaveMode, SaveQuery,
    SaveQueryDyn, SaveResponse, SaveResponseDyn,
};
pub use traits::*;
pub use types::*;

use crate::{SerializeError, ThisError, db::DbError, traits::Entity};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// QueryError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum QueryError {
    #[error(transparent)]
    DbError(#[from] DbError),

    #[error(transparent)]
    LoadError(#[from] LoadError),

    #[error(transparent)]
    SaveError(#[from] SaveError),

    #[error(transparent)]
    ResolverError(#[from] ResolverError),

    #[error(transparent)]
    SerializeError(#[from] SerializeError),
}

// load
#[must_use]
pub fn load<E: Entity>() -> LoadQueryInit<E> {
    LoadQueryInit::<E>::new()
}

// load_dyn
#[must_use]
pub fn load_dyn() -> LoadQueryDynInit {
    LoadQueryDynInit::new()
}

// delete
#[must_use]
pub fn delete<E: Entity>() -> DeleteBuilder<E> {
    DeleteBuilder::<E>::new()
}

// save
#[must_use]
pub const fn save<E: Entity>(mode: SaveMode) -> SaveBuilder<E> {
    SaveBuilder::<E>::new(mode)
}

// save_dyn
#[must_use]
pub const fn save_dyn(mode: SaveMode) -> SaveBuilderDyn {
    SaveBuilderDyn::new(mode)
}

// create
#[must_use]
pub const fn create<E: Entity>() -> SaveBuilder<E> {
    SaveBuilder::<E>::new(SaveMode::Create)
}

// create_dyn
#[must_use]
pub const fn create_dyn() -> SaveBuilderDyn {
    SaveBuilderDyn::new(SaveMode::Create)
}

// replace
#[must_use]
pub const fn replace<E: Entity>() -> SaveBuilder<E> {
    SaveBuilder::<E>::new(SaveMode::Replace)
}

// replace_dyn
#[must_use]
pub const fn replace_dyn() -> SaveBuilderDyn {
    SaveBuilderDyn::new(SaveMode::Replace)
}

// update
#[must_use]
pub const fn update<E: Entity>() -> SaveBuilder<E> {
    SaveBuilder::<E>::new(SaveMode::Update)
}

// update_dyn
#[must_use]
pub const fn update_dyn() -> SaveBuilderDyn {
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
    pub const fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn println(&self, s: &str) {
        if self.enabled {
            icu::ic::println!("{s}");
        }
    }
}
