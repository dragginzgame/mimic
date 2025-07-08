pub mod children;
pub mod self_;

use crate::{
    node::{Entity, Enum, List, Map, Newtype, Record, Set},
    node_traits::{Imp, ImpFn, Implementor, Trait},
};
use children::ValidateChildrenFunction;
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
    fn tokens(node: &Entity) -> Option<TokenStream> {
        let q = ValidateChildrenFunction::tokens(node);

        let tokens = Implementor::new(&node.def, Trait::ValidateAuto)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// Enum
///

impl Imp<Enum> for ValidateAutoTrait {
    fn tokens(node: &Enum) -> Option<TokenStream> {
        let q = ValidateSelfFunction::tokens(node);

        let tokens = Implementor::new(&node.def, Trait::ValidateAuto)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// List
///

impl Imp<List> for ValidateAutoTrait {
    fn tokens(node: &List) -> Option<TokenStream> {
        let q = ValidateChildrenFunction::tokens(node);

        let tokens = Implementor::new(&node.def, Trait::ValidateAuto)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// Map
///

impl Imp<Map> for ValidateAutoTrait {
    fn tokens(node: &Map) -> Option<TokenStream> {
        let q = ValidateChildrenFunction::tokens(node);

        let tokens = Implementor::new(&node.def, Trait::ValidateAuto)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// Newtype
///

impl Imp<Newtype> for ValidateAutoTrait {
    fn tokens(node: &Newtype) -> Option<TokenStream> {
        let q = ValidateChildrenFunction::tokens(node);

        let tokens = Implementor::new(&node.def, Trait::ValidateAuto)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// Record
///

impl Imp<Record> for ValidateAutoTrait {
    fn tokens(node: &Record) -> Option<TokenStream> {
        let q = ValidateChildrenFunction::tokens(node);

        let tokens = Implementor::new(&node.def, Trait::ValidateAuto)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// Set
///

impl Imp<Set> for ValidateAutoTrait {
    fn tokens(node: &Set) -> Option<TokenStream> {
        let q = ValidateChildrenFunction::tokens(node);

        let tokens = Implementor::new(&node.def, Trait::ValidateAuto)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}
