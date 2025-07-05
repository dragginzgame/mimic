use crate::{
    node::{Entity, Enum, FieldList, Newtype, Record},
    node_traits::{Imp, Implementor, Trait},
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;

///
/// TypeViewTrait
///

pub struct TypeViewTrait {}

///
/// Newtype
///

impl Imp<Newtype> for TypeViewTrait {
    fn tokens(node: &Newtype, t: Trait) -> Option<TokenStream> {
        let view_ident = node.def.view_ident();

        let q = quote! {
            type View = #view_ident;

            fn to_view(&self) -> Self::View {
                #view_ident(self.inner())
            }

            fn from_view(view: Self::View) -> Self {
                Self(view.0.into())
            }
        };

        Some(
            Implementor::new(&node.def, t)
                .set_tokens(q)
                .to_token_stream(),
        )
    }
}

///
/// Entity
///

impl Imp<Entity> for TypeViewTrait {
    fn tokens(node: &Entity, t: Trait) -> Option<TokenStream> {
        let view_ident = node.def.view_ident();
        let tokens = field_list(view_ident, &node.fields);

        Some(
            Implementor::new(&node.def, t)
                .set_tokens(tokens)
                .to_token_stream(),
        )
    }
}

///
/// Record
///

impl Imp<Record> for TypeViewTrait {
    fn tokens(node: &Record, t: Trait) -> Option<TokenStream> {
        let view_ident = node.def.view_ident();
        let tokens = field_list(view_ident, &node.fields);

        Some(
            Implementor::new(&node.def, t)
                .set_tokens(tokens)
                .to_token_stream(),
        )
    }
}

///
/// Helpers
///

// field_list
fn field_list(view_ident: Ident, fields: &FieldList) -> TokenStream {
    let to_pairs: Vec<_> = fields
        .iter()
        .map(|field| {
            let name = &field.name;
            quote! {
                #name: ::mimic::core::traits::TypeView::to_view(&self.#name)
            }
        })
        .collect();

    let from_pairs: Vec<_> = fields
        .iter()
        .map(|field| {
            let name = &field.name;
            quote! {
                #name: ::mimic::core::traits::TypeView::from_view(view.#name)
            }
        })
        .collect();

    quote! {
        type View = #view_ident;

        fn to_view(&self) -> Self::View {
            #view_ident {
                #(#to_pairs),*
            }
        }

        fn from_view(view: Self::View) -> Self {
            Self {
                #(#from_pairs),*
            }
        }
    }
}
