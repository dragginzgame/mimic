pub mod children;
pub mod self_;

use crate::{
    node::{Entity, Enum, List, Map, Newtype, Record, Set},
    node_traits::{Imp, ImpFn, Implementor, Trait, TraitStrategy},
    traits::HasIdent,
};
use children::ValidateChildrenFunction;
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
    fn strategy(node: &Entity) -> Option<TraitStrategy> {
        let q = ValidateChildrenFunction::tokens(node);

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
        let q = ValidateSelfFunction::tokens(node);

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
        let q = ValidateChildrenFunction::tokens(node);

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
        let q = ValidateChildrenFunction::tokens(node);

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
        let q = ValidateChildrenFunction::tokens(node);

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
        let q = ValidateChildrenFunction::tokens(node);

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
        let q = ValidateChildrenFunction::tokens(node);

        let tokens = Implementor::new(node.ident(), Trait::ValidateAuto)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}
