pub mod children;
pub mod self_;

use crate::{
    imp::{Imp, ImpFn, Implementor, Trait, TraitStrategy},
    node::{Entity, Enum, List, Map, Newtype, Record, Set},
    traits::HasIdent,
};
use children::ValidateChildrenFn;
use quote::ToTokens;
use self_::ValidateSelfFn;

///
/// ValidateAutoTrait
///

pub struct ValidateAutoTrait {}

///
/// Entity
///

impl Imp<Entity> for ValidateAutoTrait {
    fn strategy(node: &Entity) -> Option<TraitStrategy> {
        let q = ValidateChildrenFn::tokens(node);

        let tokens = Implementor::new(node.ident(), Trait::ValidateAuto)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// Enum
///

impl Imp<Enum> for ValidateAutoTrait {
    fn strategy(node: &Enum) -> Option<TraitStrategy> {
        let q = ValidateSelfFn::tokens(node);

        let tokens = Implementor::new(node.ident(), Trait::ValidateAuto)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// List
///

impl Imp<List> for ValidateAutoTrait {
    fn strategy(node: &List) -> Option<TraitStrategy> {
        let q = ValidateChildrenFn::tokens(node);

        let tokens = Implementor::new(node.ident(), Trait::ValidateAuto)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// Map
///

impl Imp<Map> for ValidateAutoTrait {
    fn strategy(node: &Map) -> Option<TraitStrategy> {
        let q = ValidateChildrenFn::tokens(node);

        let tokens = Implementor::new(node.ident(), Trait::ValidateAuto)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// Newtype
///

impl Imp<Newtype> for ValidateAutoTrait {
    fn strategy(node: &Newtype) -> Option<TraitStrategy> {
        let q = ValidateChildrenFn::tokens(node);

        let tokens = Implementor::new(node.ident(), Trait::ValidateAuto)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// Record
///

impl Imp<Record> for ValidateAutoTrait {
    fn strategy(node: &Record) -> Option<TraitStrategy> {
        let q = ValidateChildrenFn::tokens(node);

        let tokens = Implementor::new(node.ident(), Trait::ValidateAuto)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// Set
///

impl Imp<Set> for ValidateAutoTrait {
    fn strategy(node: &Set) -> Option<TraitStrategy> {
        let q = ValidateChildrenFn::tokens(node);

        let tokens = Implementor::new(node.ident(), Trait::ValidateAuto)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}
