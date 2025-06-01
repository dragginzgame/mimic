pub mod delete;
pub mod load;
pub mod resolver;
pub mod save;
pub mod traits;

pub use delete::*;
pub use load::*;
pub use resolver::*;
pub use save::*;
pub use traits::*;

use crate::{SerializeError, ThisError, db::DbError, traits::Entity};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// QueryError
///

#[derive(Debug, ThisError)]
pub enum QueryError {
    #[error(transparent)]
    DbError(#[from] DbError),

    #[error(transparent)]
    LoadError(#[from] LoadError),

    #[error(transparent)]
    SaveError(#[from] SaveError),

    #[error(transparent)]
    DeleteError(#[from] DeleteError),

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
pub const fn load_dyn() -> LoadQueryDynInit {
    LoadQueryDynInit::new()
}

// delete
#[must_use]
pub const fn delete() -> DeleteQueryInit {
    DeleteQueryInit::new()
}

// save
#[must_use]
pub const fn save() -> SaveQueryInit {
    SaveQueryInit::new()
}

// create
#[must_use]
pub const fn create() -> SaveQueryModeInit {
    SaveQueryModeInit::new(SaveMode::Create)
}

// update
#[must_use]
pub const fn update() -> SaveQueryModeInit {
    SaveQueryModeInit::new(SaveMode::Update)
}

// replace
#[must_use]
pub const fn replace() -> SaveQueryModeInit {
    SaveQueryModeInit::new(SaveMode::Replace)
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
