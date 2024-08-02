use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

///
/// Storable
/// just so the code's in one place, we can redo this in the future
/// always uses UNBOUNDED
///

#[proc_macro_derive(Storable)]
pub fn storable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl ::mimic_common::ic::structures::storable::Storable for #name {
            fn to_bytes(&self) -> ::std::borrow::Cow<[u8]> {
                ::std::borrow::Cow::Owned(::mimic_common::cbor::serialize(self).unwrap())
            }

            fn from_bytes(bytes: ::std::borrow::Cow<[u8]>) -> Self {
                ::mimic_common::cbor::deserialize(&bytes).unwrap()
            }

            const BOUND: ::mimic_common::ic::structures::storable::Bound =
                ::mimic_common::ic::structures::storable::Bound::Unbounded;
        }
    };

    TokenStream::from(expanded)
}
