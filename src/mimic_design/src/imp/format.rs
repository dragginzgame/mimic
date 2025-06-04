use crate::{
    imp::{Imp, Implementor},
    node::{Def, Entity, Enum, List, Map, Newtype, Record, Set, Trait, Tuple},
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// FormatSortKeyTrait
///

pub struct FormatSortKeyTrait {}

///
/// List
///

impl Imp<List> for FormatSortKeyTrait {
    fn tokens(node: &List, t: Trait) -> Option<TokenStream> {
        format_sort_key_none(&node.def, t)
    }
}

///
/// Entity
///

impl Imp<Entity> for FormatSortKeyTrait {
    fn tokens(node: &Entity, t: Trait) -> Option<TokenStream> {
        format_sort_key_none(&node.def, t)
    }
}

///
/// Enum
///

impl Imp<Enum> for FormatSortKeyTrait {
    fn tokens(node: &Enum, t: Trait) -> Option<TokenStream> {
        format_sort_key_none(&node.def, t)
    }
}

///
/// Map
///

impl Imp<Map> for FormatSortKeyTrait {
    fn tokens(node: &Map, t: Trait) -> Option<TokenStream> {
        format_sort_key_none(&node.def, t)
    }
}

///
/// Newtype
///

impl Imp<Newtype> for FormatSortKeyTrait {
    fn tokens(node: &Newtype, t: Trait) -> Option<TokenStream> {
        let q = quote! {
            fn format_sort_key(&self) -> Option<String> {
                self.0.format_sort_key()
            }
        };

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// Record
///

impl Imp<Record> for FormatSortKeyTrait {
    fn tokens(node: &Record, t: Trait) -> Option<TokenStream> {
        format_sort_key_none(&node.def, t)
    }
}

///
/// Set
///

impl Imp<Set> for FormatSortKeyTrait {
    fn tokens(node: &Set, t: Trait) -> Option<TokenStream> {
        format_sort_key_none(&node.def, t)
    }
}

///
/// Tuple
///

impl Imp<Tuple> for FormatSortKeyTrait {
    fn tokens(node: &Tuple, t: Trait) -> Option<TokenStream> {
        format_sort_key_none(&node.def, t)
    }
}

// format_sort_key_none
#[allow(clippy::unnecessary_wraps)]
fn format_sort_key_none(def: &Def, t: Trait) -> Option<TokenStream> {
    let q = quote! {
        fn format_sort_key(&self) -> Option<String> {
            None
        }
    };

    let tokens = Implementor::new(def, t).set_tokens(q).to_token_stream();

    Some(tokens)
}
