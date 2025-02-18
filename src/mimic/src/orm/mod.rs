pub mod base;
pub mod traits;
pub mod visit;

///
/// PRELUDE
/// using _ brings traits into scope and avoids name conflicts
///

pub mod prelude {
    pub use crate::{
        ic::structures::storable::Bound,
        orm::{
            OrmError,
            base::types::Ulid,
            traits::{
                EntityDyn, EntityFixture, EntityId as _, Filterable, Inner as _, NumCast,
                Orderable, Path, Selector as _, SortKey as _, Storable, Validate as _,
                ValidateManual, Validator, Visitable,
            },
        },
        types::{ErrorVec, FixtureList},
        utils::case::{Case, Casing},
    };
    pub use ::candid::CandidType;
    pub use ::mimic_design::*;
    pub use ::serde::{Deserialize, Serialize};
    pub use ::std::{cmp::Ordering, collections::HashSet, fmt::Display};
}

use crate::{Error, ThisError, ic::serialize::SerializeError, types::ErrorTree};
use candid::CandidType;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use traits::Visitable;
use visit::{ValidateVisitor, perform_visit};

///
/// OrmError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum OrmError {
    // entity not found, used for auto-generated endpoints
    #[error("entity not found: {0}")]
    EntityNotFound(String),

    #[error("validation failed: {0}")]
    Validation(ErrorTree),

    #[error(transparent)]
    SerializeError(#[from] SerializeError),
}

// validate
pub fn validate(node: &dyn Visitable) -> Result<(), Error> {
    let mut visitor = ValidateVisitor::new();
    perform_visit(&mut visitor, node, "");

    visitor.errors.result().map_err(OrmError::Validation)?;

    Ok(())
}

// serialize
pub fn serialize<T>(ty: &T) -> Result<Vec<u8>, OrmError>
where
    T: Serialize,
{
    crate::ic::serialize::serialize(ty).map_err(OrmError::SerializeError)
}

// deserialize
pub fn deserialize<T>(bytes: &[u8]) -> Result<T, OrmError>
where
    T: DeserializeOwned,
{
    crate::ic::serialize::deserialize(bytes).map_err(OrmError::SerializeError)
}
