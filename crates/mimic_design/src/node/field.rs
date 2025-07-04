use crate::{
    helper::{quote_one, quote_option, quote_slice, to_str_lit},
    node::{Arg, Value},
    traits::SchemaNode,
    types::Cardinality,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use std::slice::Iter;
use syn::Ident;

///
/// FieldList
///

#[derive(Clone, Debug, Default, FromMeta)]
pub struct FieldList {
    #[darling(multiple, rename = "field")]
    pub fields: Vec<Field>,
}

impl FieldList {
    pub fn iter(&self) -> std::slice::Iter<'_, Field> {
        self.fields.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Field> {
        self.fields.iter_mut()
    }

    pub fn type_view_fields(&self, view_ident: &Ident) -> TokenStream {
        let view_fields: Vec<_> = self
            .fields
            .iter()
            .map(|f| {
                let name = &f.name;
                let ty = &f.value;
                quote! {
                    #name: <#ty as ::mimic::core::traits::TypeView>::View
                }
            })
            .collect();

        quote! {
            pub struct #view_ident {
                #(#view_fields,)*
            }
        }
    }

    // has_default
    pub fn has_default(&self) -> bool {
        self.fields.iter().any(|f| f.default.is_some())
    }
}

impl<'a> IntoIterator for &'a FieldList {
    type Item = &'a Field;
    type IntoIter = Iter<'a, Field>;

    fn into_iter(self) -> Self::IntoIter {
        self.fields.iter()
    }
}

impl SchemaNode for FieldList {
    fn schema(&self) -> TokenStream {
        let fields = quote_slice(&self.fields, Field::schema);

        quote! {
            ::mimic::schema::node::FieldList {
                fields: #fields,
            }
        }
    }
}

impl ToTokens for FieldList {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let fields = &self.fields;

        tokens.extend(quote! {
            #(#fields,)*
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

    #[darling(default)]
    pub default: Option<Arg>,
}

impl SchemaNode for Field {
    fn schema(&self) -> TokenStream {
        let name = quote_one(&self.name, to_str_lit);
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

        // serde
        let serde_attr = match *value.cardinality() {
            Cardinality::Opt => {
                quote!(#[serde(default, skip_serializing_if = "Option::is_none")])
            }
            Cardinality::Many => {
                quote!(#[serde(default, skip_serializing_if = "Vec::is_empty")])
            }
            Cardinality::One => quote!(),
        };

        // build struct field
        tokens.extend(quote! {
            #serde_attr
            pub #name : #value
        });
    }
}
