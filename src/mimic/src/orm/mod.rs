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
            base::types::Ulid,
            traits::{
                EntityDyn, EntityFixture, EntityId as _, Filterable, Inner as _, NumCast,
                Orderable, Path, Selector as _, SortKey as _, Storable, Validate as _,
                ValidateManual, Validator, Visitable,
            },
            OrmError,
        },
        query::Query,
        types::{ErrorVec, FixtureList},
        utils::case::{Case, Casing},
    };
    pub use ::candid::CandidType;
    pub use ::mimic_design::*;
    pub use ::serde::{Deserialize, Serialize};
    pub use ::std::{cmp::Ordering, collections::HashSet, fmt::Display};
}

use crate::{ic::serialize::SerializeError, types::ErrorTree, Error, ThisError};
use candid::CandidType;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use traits::Visitable;
use visit::{perform_visit, ValidateVisitor};

///
/// OrmError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum OrmError {
    #[error("cannot parse field '{0}'")]
    ParseField(String),

    #[error("validation failed: {0}")]
    Validation(ErrorTree),

    #[error(transparent)]
    SerializeError(#[from] SerializeError),
}

impl OrmError {
    #[must_use]
    pub fn parse_field(field: &str) -> Self {
        Self::ParseField(field.to_string())
    }
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
