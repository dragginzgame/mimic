use crate::{
    helper::{quote_one, quote_option, to_string},
    node::{Arg, Value},
    traits::Schemable,
};
use darling::FromMeta;
use mimic_common::types::SortDirection;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;

///
/// Field
///

#[derive(Clone, Debug, FromMeta)]
pub struct Field {
    pub name: Ident,
    pub value: Value,

    #[darling(default)]
    pub default: Option<Arg>,
}

impl Schemable for Field {
    fn schema(&self) -> TokenStream {
        let name = quote_one(&self.name, to_string);
        let value = self.value.schema();
        let default = quote_option(self.default.as_ref(), Arg::schema);

        quote! {
            ::mimic::schema::node::Field {
                name: #name,
                value: #value,
                default: #default,
            }
        }
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let value = &self.value;

        // build struct field
        tokens.extend(quote! {
            pub #name : #value
        });
    }
}

///
/// FieldOrder
///

#[derive(Clone, Debug, FromMeta)]
pub struct FieldOrder {
    pub field: Ident,

    #[darling(default)]
    pub direction: SortDirection,
}

impl Schemable for FieldOrder {
    fn schema(&self) -> TokenStream {
        let field = quote_one(&self.field, to_string);
        let direction = &self.direction;

        quote! {
            ::mimic::schema::node::FieldOrder {
                field: #field,
                direction: #direction,
            }
        }
    }
}

impl ToTokens for FieldOrder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let field = &self.field.to_string();
        let direction = &self.direction;

        // bulld struct field
        tokens.extend(quote! {
            (#field.to_string(), #direction)
        });
    }
}
