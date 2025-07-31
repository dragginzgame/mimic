use crate::{
    node::Newtype,
    node_traits::{Imp, TraitStrategy},
    traits::HasTypePart,
};
use quote::quote;

///
/// PartialEqTrait
///

pub struct PartialEqTrait {}

///
/// Newtype
///

impl Imp<Newtype> for PartialEqTrait {
    fn strategy(node: &Newtype) -> Option<TraitStrategy> {
        let ident = &node.def.ident;
        let item = &node.item.type_part();

        let tokens = quote! {
            impl PartialEq<#item> for #ident {
                fn eq(&self, other: &#item) -> bool {
                    self.0 == *other
                }
            }

            impl PartialEq<#ident> for #item {
                fn eq(&self, other: &#ident) -> bool {
                    *self == other.0
                }
            }
        };

        Some(TraitStrategy::from_impl(tokens))
    }
}
