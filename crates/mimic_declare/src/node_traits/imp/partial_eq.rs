use crate::{
    node::{ItemTarget, Newtype},
    node_traits::{Imp, Implementor, Trait, TraitStrategy},
    traits::HasIdent,
};
use quote::{ToTokens, quote};

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
        let item = &node.item;

        let is_type = match item.target() {
            ItemTarget::Primitive(_) => return None, // 👈 skip if primitive
            ItemTarget::Is(ty) => ty,
        };

        let tokens = quote! {
            impl PartialEq<#is_type> for #ident {
                fn eq(&self, other: &#is_type) -> bool {
                    self.0 == *other
                }
            }

            impl PartialEq<#ident> for #is_type {
                fn eq(&self, other: &#ident) -> bool {
                    *self == other.0
                }
            }
        };

        Some(TraitStrategy::from_impl(tokens))
    }
}
