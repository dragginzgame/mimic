// @todo - rust bug
// https://github.com/intellij-rust/intellij-rust/issues/9853
// remove this from time to time to see the actual unused imports
#![allow(unused_imports)]

pub mod auth;
pub mod canister;
pub mod sanitizer;
pub mod types;
pub mod validator;

// this needs to be here so that mimic_base can be used both
// externally and internally
extern crate self as mimic_base;

// init
// schema generation requires a function stub
// to work on OSX
pub const fn init() {}
