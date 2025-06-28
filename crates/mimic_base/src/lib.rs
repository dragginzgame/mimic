// @todo - rust bug
// https://github.com/intellij-rust/intellij-rust/issues/9853
// remove this from time to time to see the actual unused imports
//#![allow(unused_imports)]

pub mod types;
pub mod validator;

///
/// Prelude
///

pub(crate) mod prelude {
    pub use crate::{types, validator};
    pub use mimic::core::{
        traits::{ValidatorBytes, ValidatorNumber, ValidatorString},
        types::*,
    };
    pub use mimic::prelude::*;
    pub use mimic_design::*;
}
