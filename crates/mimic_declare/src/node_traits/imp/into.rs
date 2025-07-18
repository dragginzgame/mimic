use crate::{node::EntityId, node_traits::Imp};
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
            impl Into<::mimic::core::types::Ulid> for #ident {
                fn into(self) -> ::mimic::core::types::Ulid {
                    self.ulid()
                }
            }
        });

        Some(q)
    }
}
