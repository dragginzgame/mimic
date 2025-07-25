use crate::{
    node::{Entity, FieldList, Record},
    node_traits::{Imp, Implementor, Trait},
    traits::HasIdent,
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// FieldListKindTrait
///

pub struct FieldListKindTrait {}

///
/// Entity
///

impl Imp<Entity> for FieldListKindTrait {
    fn tokens(node: &Entity) -> Option<TokenStream> {
        let q = field_list(&node.fields);

        let tokens = Implementor::new(node.ident(), Trait::FieldListKind)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// Record
///

impl Imp<Record> for FieldListKindTrait {
    fn tokens(node: &Record) -> Option<TokenStream> {
        let q = field_list(&node.fields);

        let tokens = Implementor::new(node.ident(), Trait::FieldListKind)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

fn field_list(fields: &FieldList) -> TokenStream {
    // Extract field names as string literals
    let field_names = fields.iter().map(|f| {
        let name = &f.ident.to_string(); // assumes HasIdent is implemented

        quote! { #name }
    });

    // Wrap into an array
    quote! {
        const FIELD_NAMES: &'static [&'static str] = &[ #(#field_names),* ];
    }
}
