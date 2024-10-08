pub mod delete;
pub mod iter;
pub mod load;
pub mod resolver;
pub mod save;
pub mod types;

pub use delete::DeleteBuilder;
pub use iter::{RowIterator, RowIteratorDynamic};
pub use load::{LoadBuilder, LoadBuilderOptions};
pub use resolver::Resolver;
pub use save::{SaveBuilder, SaveMode};
pub use types::*;

use crate::Db;
use orm::traits::Entity;
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
    Orm { source: orm::Error },
}

///
/// Query Builders
///

// load
#[must_use]
pub fn load<E>(db: &Db) -> LoadBuilder<E>
where
    E: Entity + 'static,
{
    LoadBuilder::<E>::new(db)
}

// delete
#[must_use]
pub fn delete<E>(db: &Db) -> DeleteBuilder<E>
where
    E: Entity,
{
    DeleteBuilder::<E>::new(db)
}

// create
#[must_use]
pub fn create(db: &Db) -> SaveBuilder {
    SaveBuilder::new(db, SaveMode::Create)
}

// replace
#[must_use]
pub fn replace(db: &Db) -> SaveBuilder {
    SaveBuilder::new(db, SaveMode::Replace)
}

// update
#[must_use]
pub fn update(db: &Db) -> SaveBuilder {
    SaveBuilder::new(db, SaveMode::Update)
}

///
/// DebugContext
///

#[derive(Default)]
pub struct DebugContext {
    enabled: bool,
}

impl DebugContext {
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn println(&self, s: &str) {
        if self.enabled {
            ic::println!("{s}");
        }
    }
}
