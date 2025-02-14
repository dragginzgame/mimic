pub mod delete;
pub mod load;
pub mod resolver;
pub mod save;
pub mod types;

pub use delete::{
    DeleteBuilder, DeleteBuilderPath, DeleteError, DeleteExecutor, DeleteExecutorPath, DeleteQuery,
    DeleteQueryPath, DeleteResponse,
};
pub use load::{
    LoadBuilder, LoadBuilderPath, LoadError, LoadExecutor, LoadExecutorPath, LoadQuery,
    LoadQueryPath, LoadResponse, LoadResult, LoadResultDyn,
};
pub use resolver::{Resolver, ResolverError};
pub use save::{
    SaveBuilder, SaveBuilderDyn, SaveError, SaveMode, SaveQuery, SaveQueryDyn, SaveResponse,
};
pub use types::*;

use crate::{orm::traits::Entity, ThisError};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

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

///
/// Query
/// a builder that's generic over Entity
///

#[derive(Default)]
pub struct Query<E>
where
    E: Entity,
{
    phantom: PhantomData<E>,
}

impl<E> Query<E>
where
    E: Entity,
{
    #[must_use]
    pub const fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }

    #[must_use]
    pub fn load() -> LoadBuilder<E> {
        LoadBuilder::new()
    }

    // delete
    #[must_use]
    pub fn delete() -> DeleteBuilder<E> {
        DeleteBuilder::new()
    }

    // create
    #[must_use]
    pub fn create() -> SaveBuilder<E> {
        SaveBuilder::new(SaveMode::Create)
    }

    // replace
    #[must_use]
    pub fn replace() -> SaveBuilder<E> {
        SaveBuilder::new(SaveMode::Replace)
    }

    // update
    #[must_use]
    pub fn update() -> SaveBuilder<E> {
        SaveBuilder::new(SaveMode::Update)
    }
}

///
/// Other Query Builders
///

// load_path
#[must_use]
pub fn load_path() -> LoadBuilderPath {
    LoadBuilderPath::new()
}

// delete_path
#[must_use]
pub fn delete_path() -> DeleteBuilderPath {
    DeleteBuilderPath::new()
}

// create_dyn
#[must_use]
pub fn create_dyn() -> SaveBuilderDyn {
    SaveBuilderDyn::new(SaveMode::Create)
}

// replace_dyn
#[must_use]
pub fn replace_dyn() -> SaveBuilderDyn {
    SaveBuilderDyn::new(SaveMode::Replace)
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
