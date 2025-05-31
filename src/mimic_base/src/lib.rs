// @todo - rust bug
// https://github.com/intellij-rust/intellij-rust/issues/9853
// remove this from time to time to see the actual unused imports
//#![allow(unused_imports)]

pub mod types;
pub mod validator;

///
/// Prelude
///

pub mod prelude {
    pub use crate::types;
    pub use crate::validator;
    pub use mimic::prelude::*;
    pub use mimic_design::*;
}
