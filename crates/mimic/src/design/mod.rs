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
                EntityIdKind as _, EntityKind, FieldValue as _, Inner as _, Path as _,
                Sanitize as _, Sanitizer as _, Serialize as _, TypeView as _, Validate as _,
                ValidateCustom, Validator as _, Visitable as _,
            },
            types::*,
        },
        db::{self, Db},
    };
    pub use ::candid::CandidType;
    pub use derive_more;
    pub use mimic_declare::*;
}
