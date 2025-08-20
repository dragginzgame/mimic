use crate::{
    imp::{Imp, TraitStrategy},
    node::{EntityId, Selector},
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
            let variant_ident = variant.ident();
            let value = &variant.value;
            let ty = &variant.value.as_type();

            quote! { #ident::#variant_ident => <#ty as Into<#target>>::into(#value) }
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
