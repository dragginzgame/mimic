use proc_macro2::TokenStream;
use quote::{format_ident, quote};

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

    // ctor_schema
    // formats the code needed to send something via ctor to the schema
    #[must_use]
    fn ctor_schema(&self) -> TokenStream {
        let ctor_fn = format_ident!("ctor_{}", ic::rand::next_u64());
        let schema = self.schema();

        quote! {
            #[cfg(not(target_arch = "wasm32"))]
            #[::mimic::export::ctor::ctor]
            fn #ctor_fn() {
                ::mimic::orm::schema::build::schema_write().add_node(
                    #schema
                );
            }
        }
    }
}
