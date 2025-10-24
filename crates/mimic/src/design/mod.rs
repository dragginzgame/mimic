pub mod base;

///
/// MIMIC DESIGN PRELUDE
///
/// only meant to be used in mimic design/ directories
/// it imports too much otherwise
///

pub mod prelude {
    pub use crate::{
        common::error::ErrorTree,
        core::{
            Key, Value,
            traits::{
                EntityIdKind as _, EntityKind, FieldValue as _, Inner as _, Path as _,
                Sanitize as _, Sanitizer as _, Serialize as _, TypeView as _, Validate as _,
                ValidateCustom, Validator as _, Visitable as _,
            },
            view::View,
        },
        db::{self, Db},
        types::*,
    };
    pub use ::candid::CandidType;
    pub use derive_more;
    pub use mimic_declare::*;
}
