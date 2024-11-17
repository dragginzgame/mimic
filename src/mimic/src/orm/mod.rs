pub mod base;
pub mod helper;
pub mod schema;
pub mod traits;
pub mod types;
pub mod visit;

use crate::{
    ic::structures::serialize::{from_binary, to_binary},
    orm::types::ErrorTree,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use snafu::Snafu;
use traits::Visitable;
use visit::{perform_visit, perform_visit_mut, SanitizeVisitor, ValidateVisitor};

///
/// PRELUDE
/// using _ brings traits into scope and avoids name conflicts
///

pub mod prelude {
    pub use crate::{
        ic::structures::storable::Bound,
        orm::{
            base::types::{self, Ulid},
            helper::FixtureList,
            traits::{
                EntityDyn, EntityFixture, EntityId as _, Filterable, Inner as _, Orderable, Path,
                PrimaryKey as _, Sanitize as _, SanitizeManual, Sanitizer, Storable, Validate as _,
                ValidateManual, Validator, Visitable,
            },
            types::ErrorVec,
            Error,
        },
        utils::case::{Case, Casing},
    };
    pub use ::candid::{CandidType, Principal};
    pub use ::mimic_macros::*;
    pub use ::serde::{Deserialize, Serialize};
    pub use ::snafu::Snafu;
    pub use ::std::{cmp::Ordering, collections::HashSet, fmt::Display};
}

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("cannot parse field '{field}'"))]
    ParseField { field: String },

    #[snafu(display("validation failed: {errors}"))]
    Validation { errors: ErrorTree },

    #[snafu(transparent)]
    Serialize {
        source: crate::ic::structures::serialize::Error,
    },
}

impl Error {
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
pub fn serialize<T>(ty: &T) -> Result<Vec<u8>, Error>
where
    T: Serialize,
{
    to_binary::<T>(ty).map_err(Error::from)
}

// deserialize
pub fn deserialize<T>(bytes: &[u8]) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    from_binary::<T>(bytes).map_err(Error::from)
}

// sanitize
pub fn sanitize(node: &mut dyn Visitable) {
    let mut visitor = SanitizeVisitor::new();

    perform_visit_mut(&mut visitor, node, "");
}

// validate
pub fn validate(node: &dyn Visitable) -> Result<(), Error> {
    let mut visitor = ValidateVisitor::new();
    perform_visit(&mut visitor, node, "");

    visitor
        .errors
        .result()
        .map_err(|errors| Error::Validation { errors })
}
