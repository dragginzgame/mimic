pub mod base;

///
/// MIMIC DESIGN PRELUDE
///

pub mod prelude {
    pub use crate::{
        common::{
            error::ErrorTree,
            utils::case::{Case, Casing},
        },
        core::{
            Key, Value,
            traits::{
                EntityFixture, EntityIdKind as _, EntityKind as _, Inner as _, NumCast, Path as _,
                Serialize as _, TypeView as _, Validate as _, ValidateCustom, ValidatorBytes as _,
                ValidatorDecimal as _, ValidatorNumber as _, ValidatorString as _, Visitable as _,
            },
            types::*,
        },
        db,
        db::{Db, service::EntityService},
    };
    pub use ::candid::CandidType;
    pub use derive_more;
    pub use mimic_declare::*;
}
