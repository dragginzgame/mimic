use crate::{
    node::{EntityId, Selector},
    node_traits::{Imp, TraitStrategy},
};
use quote::quote;

///
/// IntoTrait
///

pub struct IntoTrait {}

///
/// EntityId
///

impl Imp<EntityId> for IntoTrait {
    fn strategy(node: &EntityId) -> Option<TraitStrategy> {
        let ident = &node.def.ident;

        // into ulid
        let tokens = quote! {
            impl Into<::mimic::core::types::Ulid> for #ident {
                fn into(self) -> ::mimic::core::types::Ulid {
                    self.ulid()
                }
            }
        };

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// Selector
///

impl Imp<Selector> for IntoTrait {
    fn strategy(node: &Selector) -> Option<TraitStrategy> {
        let ident = &node.def.ident;
        let target = &node.target;

        // arms
        let arms = node.variants.iter().map(|variant| {
            let name = &variant.name;
            let value = &variant.value;
            let ty = &variant.value.as_type();

            quote! { #ident::#name => <#ty as Into<#target>>::into(#value) }
        });

        // into ulid
        let tokens = quote! {
            impl Into<#target> for #ident {
                fn into(self) -> #target {
                    match self {
                        #(#arms),*
                    }
                }
            }
        };

        Some(TraitStrategy::from_impl(tokens))
    }
}
