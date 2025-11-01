use proc_macro2::TokenStream;
use quote::quote;

///
/// HasType
///
/// A node that emits a Rust type definition.
///

pub trait HasType {
    /// Emit the main Rust type definition (struct, enum, etc.)
    fn type_part(&self) -> TokenStream {
        quote!()
    }
}

///
/// HasTypeExpr
///

pub trait HasTypeExpr {
    fn type_expr(&self) -> TokenStream {
        quote!()
    }
}
