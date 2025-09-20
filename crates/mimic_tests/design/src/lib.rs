pub mod admin;
pub mod e2e;
pub mod fixture;
pub mod schema;
pub mod test;

///
/// Prelude
///

pub(crate) mod prelude {
    pub use crate::{
        assert_invalid, assert_valid,
        schema::{TestDataStore, TestIndexStore},
    };
    pub use mimic::design::{
        base::{sanitizer, types, validator},
        prelude::*,
    };
}

#[macro_export]
macro_rules! assert_valid {
    ($value:expr) => {
        assert!(
            mimic::core::validate(&$value).is_ok(),
            "expected valid: {:?}",
            &$value
        );
    };
}

#[macro_export]
macro_rules! assert_invalid {
    ($value:expr) => {
        assert!(
            mimic::core::validate(&$value).is_err(),
            "expected invalid: {:?}",
            &$value
        );
    };
}
