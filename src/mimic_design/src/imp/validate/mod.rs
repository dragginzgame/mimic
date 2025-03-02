pub mod children;
pub mod self_;

use crate::{
    imp::{Imp, ImpFn, Implementor},
    node::{Entity, Enum, List, Map, Newtype, Record, Set, Trait},
};
use children::ValidateChildFunction;
use proc_macro2::TokenStream;
use quote::ToTokens;
use self_::ValidateSelfFunction;

///
/// ValidateAutoTrait
///

pub struct ValidateAutoTrait {}

///
/// Entity
///

impl Imp<Entity> for ValidateAutoTrait {
    fn tokens(node: &Entity, t: Trait) -> Option<TokenStream> {
        let q = ValidateChildFunction::tokens(node);

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// Enum
///

impl Imp<Enum> for ValidateAutoTrait {
    fn tokens(node: &Enum, t: Trait) -> Option<TokenStream> {
        let q = ValidateSelfFunction::tokens(node);

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// Record
///

impl Imp<Record> for ValidateAutoTrait {
    fn tokens(node: &Record, t: Trait) -> Option<TokenStream> {
        let q = ValidateChildFunction::tokens(node);

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// List
///

impl Imp<List> for ValidateAutoTrait {
    fn tokens(node: &List, t: Trait) -> Option<TokenStream> {
        let q = ValidateChildFunction::tokens(node);

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// Map
///

impl Imp<Map> for ValidateAutoTrait {
    fn tokens(node: &Map, t: Trait) -> Option<TokenStream> {
        let q = ValidateChildFunction::tokens(node);

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// Newtype
///

impl Imp<Newtype> for ValidateAutoTrait {
    fn tokens(node: &Newtype, t: Trait) -> Option<TokenStream> {
        let q = ValidateChildFunction::tokens(node);

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// Set
///

impl Imp<Set> for ValidateAutoTrait {
    fn tokens(node: &Set, t: Trait) -> Option<TokenStream> {
        let q = ValidateChildFunction::tokens(node);

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}
