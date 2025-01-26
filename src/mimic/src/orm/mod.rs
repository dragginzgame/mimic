pub mod base;
pub mod schema;
pub mod traits;
pub mod types;
pub mod visit;

use crate::{
    ic::structures::serialize::{from_binary, to_binary, SerializeError},
    orm::types::ErrorTree,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use snafu::Snafu;
use traits::Visitable;
use visit::{perform_visit, ValidateVisitor};

///
/// PRELUDE
/// using _ brings traits into scope and avoids name conflicts
///

pub mod prelude {
    pub use crate::{
        ic::structures::storable::Bound,
        orm::{
            traits::{
                EntityDyn, EntityFixture, EntityId as _, Filterable, Inner as _, NumCast,
                Orderable, Path, Selector as _, SortKey as _, Storable, Validate as _,
                ValidateManual, Validator, Visitable,
            },
            types::ErrorVec,
            OrmError,
        },
        utils::case::{Case, Casing},
    };
    pub use ::candid::CandidType;
    pub use ::mimic_macros::*;
    pub use ::serde::{Deserialize, Serialize};
    pub use ::snafu::Snafu;
    pub use ::std::{cmp::Ordering, collections::HashSet, fmt::Display};
}

///
/// OrmError
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum OrmError {
    #[snafu(display("cannot parse field '{field}'"))]
    ParseField { field: String },

    #[snafu(display("validation failed: {errors}"))]
    Validation { errors: ErrorTree },

    #[snafu(transparent)]
    SerializeError { source: SerializeError },
}

impl OrmError {
    #[must_use]
    pub fn parse_field(field: &str) -> Self {
        Self::ParseField {
            field: field.to_string(),
        }
    }
}

///
/// TYPE FUNCTIONS
/// The primary functions to validate and manipulate types within the ORM
///

// serialize
pub fn serialize<T>(ty: &T) -> Result<Vec<u8>, OrmError>
where
    T: Serialize,
{
    to_binary::<T>(ty).map_err(OrmError::from)
}

// deserialize
pub fn deserialize<T>(bytes: &[u8]) -> Result<T, OrmError>
where
    T: DeserializeOwned,
{
    from_binary::<T>(bytes).map_err(OrmError::from)
}

// validate
pub fn validate(node: &dyn Visitable) -> Result<(), OrmError> {
    let mut visitor = ValidateVisitor::new();
    perform_visit(&mut visitor, node, "");

    visitor
        .errors
        .result()
        .map_err(|errors| OrmError::Validation { errors })
}
