pub mod delete;
pub mod iter;
pub mod load;
pub mod resolver;
pub mod save;
pub mod types;

pub use delete::{DeleteBuilder, DeleteExecutor, DeleteQuery, DeleteResponse};
pub use iter::{ERowIterator, RowIterator};
pub use load::{ELoadBuilder, ELoadExecutor, ELoadQuery, LoadBuilder, LoadExecutor, LoadQuery};
pub use resolver::Resolver;
pub use save::{SaveBuilder, SaveMode, SaveResponse};
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
