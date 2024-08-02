pub mod build;
pub mod helper;
pub mod node;
pub mod types;
pub mod visit;

use proc_macro2::TokenStream;

///
/// Schemable
///
/// Any data structure requires this trait to be part of the ctor structure
/// that populates the Schema
///
pub trait Schemable {
    // schema
    // generates the structure which is passed to the static Schema data structure
    // via the ctor crate
    fn schema(&self) -> TokenStream;
}
