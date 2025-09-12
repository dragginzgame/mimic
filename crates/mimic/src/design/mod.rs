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
                EntityFixture, EntityIdKind as _, EntityKind, FieldValue as _, NumToPrimitive as _,
                Path as _, Serialize as _, TypeView as _, Validate as _, ValidateCustom,
                Validator as _, Visitable as _,
            },
            types::*,
        },
        db::{self, Db, service::EntityService},
    };
    pub use ::candid::CandidType;
    pub use derive_more;
    pub use mimic_declare::*;
}
