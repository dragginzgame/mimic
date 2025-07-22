use crate::{
    node::{EntityId, Selector},
    node_traits::Imp,
};
use proc_macro2::TokenStream;
use quote::quote;

///
/// IntoTrait
///

pub struct IntoTrait {}

///
/// EntityId
///

impl Imp<EntityId> for IntoTrait {
    fn tokens(node: &EntityId) -> Option<TokenStream> {
        let ident = &node.def.ident;

        // into ulid
        Some(quote! {
            impl Into<::mimic::core::types::Ulid> for #ident {
                fn into(self) -> ::mimic::core::types::Ulid {
                    self.ulid()
                }
            }
        })
    }
}

///
/// Selector
///

impl Imp<Selector> for IntoTrait {
    fn tokens(node: &Selector) -> Option<TokenStream> {
        let ident = &node.def.ident;
        let target = &node.target;

        // arms
        let arms = node.variants.iter().map(|variant| {
            let name = &variant.name;
            let value = &variant.value;

            quote! { #ident::#name => #value }
        });

        // into ulid
        Some(quote! {
            impl Into<#target> for #ident {
                fn into(self) -> #target {
                    match self {
                        #(#arms),*
                    }
                }
            }
        })
    }
}
