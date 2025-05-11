///
/// mimic
/// [for external use only, keep out of reach of children]
///
pub mod db;
pub mod interface;
pub mod macros;
pub mod orm;
pub mod query;
pub mod schema;
pub mod types;
pub mod utils;

// makes it easier to use internally
pub mod ic {
    pub use icu::ic::*;
}

pub mod export {
    pub use ctor;
    pub use derive_more;
    pub use num_traits;
    pub use remain;
}

extern crate self as mimic;

///
/// MIMIC PRELUDE
/// using _ brings traits into scope and avoids name conflicts
///

pub mod prelude {
    pub use crate::{
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
    pub use ::mimic_design::*;
}

use candid::CandidType;
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

///
/// Error
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum Error {
    #[error(transparent)]
    DbError(#[from] db::DbError),

    #[error(transparent)]
    InterfaceError(#[from] interface::InterfaceError),

    #[error(transparent)]
    OrmError(#[from] orm::OrmError),

    #[error(transparent)]
    QueryError(#[from] query::QueryError),

    #[error(transparent)]
    SchemaError(#[from] schema::SchemaError),
}
