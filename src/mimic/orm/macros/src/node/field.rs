use crate::{
    helper::{quote_one, quote_vec, to_string},
    node::Value,
};
use darling::FromMeta;
use orm::types::SortDirection;
use orm_schema::Schemable;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Ident;

///
/// FieldList
///
/// display is fine until we have to go and redo it for the whole caching system
///

#[derive(Clone, Debug, Default, FromMeta)]
pub struct FieldList {
    #[darling(multiple, rename = "field")]
    pub fields: Vec<Field>,

    #[darling(multiple)]
    pub order: Vec<FieldOrder>,
}

impl FieldList {
    pub fn has_default(&self) -> bool {
        self.fields.iter().any(|f| f.value.default.is_some())
    }
}

impl Schemable for FieldList {
    fn schema(&self) -> TokenStream {
        let fields = quote_vec(&self.fields, Field::schema);
        let order = quote_vec(&self.order, FieldOrder::schema);

        quote! {
            ::mimic::orm::schema::node::FieldList {
                fields: #fields,
                order: #order,
            }
        }
    }
}

impl ToTokens for FieldList {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let fields = &self.fields;

        tokens.extend(quote! {
            #(#fields)*
        });
    }
}

///
/// Field
///

#[derive(Clone, Debug, FromMeta)]
pub struct Field {
    pub name: Ident,
    pub value: Value,
}

impl Schemable for Field {
    fn schema(&self) -> TokenStream {
        let name = quote_one(&self.name, to_string);
        let value = self.value.schema();

        quote! {
            ::mimic::orm::schema::node::Field {
                name: #name,
                value: #value,
            }
        }
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let value = &self.value;

        // bulld struct field
        tokens.extend(quote! {
            pub #name : #value,
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
        let direction = self.direction.schema();

        quote! {
            ::mimic::orm::schema::node::FieldOrder {
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
