pub mod delete;
pub mod iter;
pub mod load;
pub mod resolver;
pub mod save;
pub mod types;

pub use delete::DeleteBuilder;
pub use iter::{RowIterator, RowIteratorDyn};
pub use load::{
    LoadBuilder, LoadBuilderDyn, LoadExecutor, LoadExecutorDyn, LoadQuery, LoadQueryDyn,
};
pub use resolver::Resolver;
pub use save::{SaveBuilder, SaveMode, SaveResult};
pub use types::*;

use crate::orm::traits::Entity;
use candid::CandidType;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(transparent)]
    Delete { source: delete::Error },

    #[snafu(transparent)]
    Load { source: load::Error },

    #[snafu(transparent)]
    Save { source: save::Error },

    #[snafu(transparent)]
    Iter { source: iter::Error },

    #[snafu(transparent)]
    Orm { source: crate::orm::Error },
}

///
/// Query Builders
///

// load
#[must_use]
pub fn load<E>() -> LoadBuilder<E>
where
    E: Entity + 'static,
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

// create
#[must_use]
pub fn create() -> SaveBuilder {
    SaveBuilder::new(SaveMode::Create)
}

// replace
#[must_use]
pub fn replace() -> SaveBuilder {
    SaveBuilder::new(SaveMode::Replace)
}

// update
#[must_use]
pub fn update() -> SaveBuilder {
    SaveBuilder::new(SaveMode::Update)
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
