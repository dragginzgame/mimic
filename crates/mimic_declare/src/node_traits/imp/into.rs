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
        let mut q = quote!();

        // into ulid
        q.extend(quote! {
            impl Into<Ulid> for #ident {
                fn into(self) -> mimic::core::types::Ulid {
                    self.ulid()
                }
            }
        });

        // into relation
        q.extend(quote! {
            impl Into<Key> for #ident {
                fn into(self) -> mimic::core::Key {
                    self.ulid().into()
                }
            }
        });

        Some(q)
    }
}

///
/// Selector
///

impl Imp<Selector> for IntoTrait {
    fn tokens(node: &Selector) -> Option<TokenStream> {
        let ident = &node.def.ident;
        let target = &node.target;

        // build match arms for each variant
        let match_arms = node.variants.iter().map(|variant| {
            let name = &variant.name;
            let value = &variant.value;

            quote! {
                Self::#name => <#target as ::std::convert::From<_>>::from(#value),
            }
        });

        // Into
        let q = quote! {
            impl Into<#target> for #ident {
                fn into(self) -> #target {
                    match self {
                        #(#match_arms)*
                    }
                }
            }
        };

        Some(q)
    }
}
