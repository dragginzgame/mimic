mod default;
mod entity;
mod enum_value;
mod field;
mod implementor;
mod inner;
mod num;
mod type_view;
mod validate;
mod visitable;

pub use default::*;
pub use entity::*;
pub use enum_value::*;
pub use field::*;
pub use implementor::*;
pub use inner::*;
pub use num::*;
pub use type_view::*;
pub use validate::*;
pub use visitable::*;

use crate::{node_traits::Trait, traits::MacroNode};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// any
///
/// shared implementation
/// that can be used by a Node of any type
///

pub fn any<N: MacroNode>(node: &N, t: Trait) -> Option<TokenStream> {
    let def = node.def();

    match t {
        Trait::Path => {
            let ident_str = format!("{}", def.ident);
            let q = quote! {
                const PATH: &'static str = concat!(module_path!(), "::", #ident_str);
            };

            Some(Implementor::new(def, t).set_tokens(q).to_token_stream())
        }

        // empty implementations are generated for these traits
        Trait::EntityFixture
        | Trait::EntityIdKind
        | Trait::FieldSearchable
        | Trait::FieldSortable
        | Trait::FieldValue
        | Trait::ValidateAuto
        | Trait::ValidateCustom
        | Trait::Visitable => Some(Implementor::new(def, t).to_token_stream()),

        _ => None,
    }
}

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
