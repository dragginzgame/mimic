pub mod default;
pub mod display;
pub mod entity;
pub mod enum_value;
pub mod filterable;
pub mod from;
pub mod implementor;
pub mod inner;
pub mod num;
pub mod orderable;
pub mod primary_key;
pub mod record_filter;
pub mod record_sort;
pub mod sanitize_auto;
pub mod validate_auto;
pub mod visitable;

use crate::node::{MacroNode, Trait};
use implementor::Implementor;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

///
/// any
///
/// shared implementation
/// that can be used by a Node of any type
///

pub fn any<N: MacroNode>(node: &N, t: Trait) -> TokenStream {
    let def = node.def();

    let imp = match t {
        Trait::NodeDyn => {
            let q = quote! {
                fn path_dyn(&self) -> String {
                    <Self as ::mimic::orm::traits::Path>::path()
                }
            };

            Implementor::new(def, t).set_tokens(q).to_token_stream()
        }

        Trait::Path => {
            let ident_str = format!("{}", def.ident);
            let q = quote! {
                const IDENT: &'static str = #ident_str;
                const PATH: &'static str = concat!(module_path!(), "::", #ident_str);
            };

            Implementor::new(def, t).set_tokens(q).to_token_stream()
        }

        Trait::Storable => {
            let q = quote! {
                fn to_bytes(&self) -> ::std::borrow::Cow<[u8]> {
                    let serialized_data = ::mimic::orm::serialize(self).unwrap();
                    ::std::borrow::Cow::Owned(serialized_data)
                }

                fn from_bytes(bytes: ::std::borrow::Cow<[u8]>) -> Self {
                    ::mimic::orm::deserialize(&bytes).unwrap()
                }

                const BOUND: ::ic::storage::storable::Bound = ::ic::storage::storable::Bound::Unbounded;
            };

            Implementor::new(def, t).set_tokens(q).to_token_stream()
        }

        // empty implementations are generated for these traits
        Trait::EntityFixture
        | Trait::EntityKey
        | Trait::Filterable
        | Trait::Orderable
        | Trait::SanitizeManual
        | Trait::SanitizeAuto
        | Trait::ValidateManual
        | Trait::ValidateAuto
        | Trait::Visitable => Implementor::new(def, t).to_token_stream(),

        _ => quote!(),
    };

    imp
}
