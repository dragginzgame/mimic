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
            base::types::{
                Blob, Bool, Decimal, Float32, Float64, Int, Int8, Int16, Int32, Int64, Int128, Nat,
                Nat8, Nat16, Nat32, Nat64, Nat128, Principal, Relation, RelationSet, Text, Ulid,
            },
            traits::{
                EntityDyn, EntityFixture, EntityId as _, Filterable, Inner as _, NumCast,
                Orderable, Ordering, Path, Selector as _, SortKeyValue as _, Validate as _,
                ValidateCustom, ValidatorBytes, ValidatorNumber, ValidatorString, Visitable,
            },
        },
        types::{ErrorTree, FixtureList},
    };
    pub use ::candid::CandidType;
    pub use ::icu::{Log, log};
    pub use ::mimic_design::*;
}

use crate::{Error, ThisError, types::ErrorTree};
use candid::CandidType;
use icu::serialize::SerializeError;
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
    icu::serialize::serialize(ty).map_err(OrmError::SerializeError)
}

// deserialize
pub fn deserialize<T>(bytes: &[u8]) -> Result<T, OrmError>
where
    T: DeserializeOwned,
{
    icu::serialize::deserialize(bytes).map_err(OrmError::SerializeError)
}
