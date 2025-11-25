pub mod sanitizer;
pub mod types;
pub mod validator;

///
/// IcyDB DESIGN PRELUDE
///
/// only meant to be used in mimic design/ directories
/// it imports too much otherwise
///

pub mod prelude {
    pub use ::candid::CandidType;
    pub use derive_more;
    pub use icydb_core::{
        Key, Value,
        db::{self, Db},
        traits::{
            EntityKind, FieldValue as _, Inner as _, Path as _, Sanitize as _, Sanitizer as _,
            Serialize as _, Validate as _, ValidateCustom, Validator as _, View as _,
            Visitable as _,
        },
        types::*,
        view::View,
    };
    pub use icydb_macros::*;
}
