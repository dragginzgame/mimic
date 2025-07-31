use crate::{
    node::Newtype,
    node_traits::{Imp, Implementor, Trait, TraitStrategy},
    traits::HasIdent,
};
use quote::{ToTokens, quote};

///
/// FieldValueTrait
///

pub struct FieldValueTrait {}

///
/// Newtype
///

impl Imp<Newtype> for FieldValueTrait {
    fn strategy(node: &Newtype) -> Option<TraitStrategy> {
        let q = quote! {
            fn to_value(&self) -> ::mimic::core::value::Value {
                self.0.to_value()
            }
        };

        let tokens = Implementor::new(node.ident(), Trait::FieldValue)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}
