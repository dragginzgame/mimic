mod as_ref;
mod default;
mod entity;
mod enum_value;
mod from;
mod inner;
mod into;
mod num;
mod orderable;
mod searchable;
mod sort_key_value;
mod validate;
mod visitable;

pub mod implementor;

pub use as_ref::*;
pub use default::*;
pub use entity::*;
pub use enum_value::*;
pub use from::*;
pub use inner::*;
pub use into::*;
pub use num::*;
pub use orderable::*;
pub use searchable::*;
pub use sort_key_value::*;
pub use validate::*;
pub use visitable::*;

use crate::node::{MacroNode, Trait};
use implementor::Implementor;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// Imp
///

pub trait Imp<N: MacroNode> {
    fn tokens(node: &N, t: Trait) -> Option<TokenStream>;
}

///
/// ImpFn
/// for breaking down traits even further
///

pub trait ImpFn<N: MacroNode> {
    fn tokens(node: &N) -> TokenStream;
}

///
/// any
///
/// shared implementation
/// that can be used by a Node of any type
///

pub fn any<N: MacroNode>(node: &N, t: Trait) -> Option<TokenStream> {
    let def = node.def();

    match t {
        Trait::NodeDyn => {
            let q = quote! {
                fn path_dyn(&self) -> String {
                    <Self as ::mimic::traits::Path>::path()
                }
            };

            Some(Implementor::new(def, t).set_tokens(q).to_token_stream())
        }

        Trait::Path => {
            let ident_str = format!("{}", def.ident);
            let q = quote! {
                const IDENT: &'static str = #ident_str;
                const PATH: &'static str = concat!(module_path!(), "::", #ident_str);
            };

            Some(Implementor::new(def, t).set_tokens(q).to_token_stream())
        }

        // empty implementations are generated for these traits
        Trait::EntityFixture
        | Trait::EntityId
        | Trait::Orderable
        | Trait::Searchable
        | Trait::ValidateAuto
        | Trait::ValidateCustom
        | Trait::Visitable => Some(Implementor::new(def, t).to_token_stream()),

        _ => None,
    }
}
