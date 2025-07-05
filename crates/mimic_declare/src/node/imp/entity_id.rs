use crate::node::EntityId;
use proc_macro2::TokenStream;
use quote::quote;

///
/// EntityId
///

pub fn tokens(node: &EntityId) -> Option<TokenStream> {
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
        impl Into<Ulid> for #ident {
            fn into(self) -> mimic::core::types::Ulid {
                self.entity_key()

            }
        }
    });

    Some(q)
}
