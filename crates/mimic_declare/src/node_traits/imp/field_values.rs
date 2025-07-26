use crate::{
    node::Entity,
    node_traits::{Imp, Implementor, Trait},
    traits::HasIdent,
};
use mimic_schema::types::Cardinality;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::LitStr;

///
/// FieldValuesTrait
///

pub struct FieldValuesTrait {}

///
/// Entity
///

impl Imp<Entity> for FieldValuesTrait {
    fn tokens(node: &Entity) -> Option<TokenStream> {
        let match_arms = node
            .fields
            .iter()
            .map(|field| {
                let field_ident = &field.ident;
                let field_lit = LitStr::new(&field_ident.to_string(), Span::call_site());

                match field.value.cardinality() {
                    Cardinality::One => Some(quote! {
                        #field_lit => Some(self.#field_ident.to_value()),
                    }),

                    Cardinality::Opt => Some(quote! {
                        #field_lit =>
                                self.#field_ident
                                    .as_ref()
                                    .map(|v| v.to_value()),
                    }),

                    Cardinality::Many => Some(quote! {
                                #field_lit => {
                                    let list = self.#field_ident
                                        .iter()
                                        .map(|v| Box::new(v.to_value()))
                                        .collect::<Vec<_>>();

                                Some(Value::List(list))
                    }}),
                }
            })
            .collect::<Vec<_>>();

        let q = quote! {
            fn get_value(&self, field: &str) -> Option<::mimic::core::Value> {
                match field {
                    #(#match_arms)*
                    _ => None,
                }
            }
        };

        let tokens = Implementor::new(node.ident(), Trait::FieldValues)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}
