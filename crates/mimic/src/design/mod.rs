pub mod base;

///
/// MIMIC DESIGN PRELUDE
///

pub mod prelude {
    pub use crate::{
        core::{
            traits::{
                EntityFixture, EntityIdKind as _, EntityKind as _, Inner as _, NumCast, Path as _,
                Serialize as _, Validate as _, ValidateCustom, ValidatorBytes, ValidatorNumber,
                ValidatorString, Visitable as _,
            },
            types::{Decimal, Principal, Ulid},
        },
        db,
        db::{executor::SaveExecutor, service::EntityService},
        error::ErrorTree,
    };
    pub use ::candid::CandidType;
    pub use mimic_design::*;
}
