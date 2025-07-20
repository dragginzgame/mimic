use crate::{
    helper::{quote_one, quote_option, quote_slice, to_str_lit},
    node::{Arg, Value},
    traits::{AsSchema, AsType},
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use std::slice::Iter;
use syn::Ident;

///
/// FieldList
///

#[derive(Debug, Default, FromMeta)]
pub struct FieldList {
    #[darling(multiple, rename = "field")]
    pub fields: Vec<Field>,
}

impl FieldList {
    pub fn get(&self, ident: &Ident) -> Option<&Field> {
        self.fields.iter().find(|f| f.ident == *ident)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Field> {
        self.fields.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Field> {
        self.fields.iter_mut()
    }

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

impl AsSchema for FieldList {
    const EMIT_SCHEMA: bool = false;

    fn schema(&self) -> TokenStream {
        let fields = quote_slice(&self.fields, Field::schema);

        quote! {
            ::mimic::schema::node::FieldList {
                fields: #fields,
            }
        }
    }
}

impl AsType for FieldList {
    fn as_type(&self) -> Option<TokenStream> {
        let fields = &self.fields;

        Some(quote! {
            #(#fields,)*
        })
    }

    fn as_view_type(&self) -> Option<TokenStream> {
        let view_fields = self.fields.iter().map(AsType::as_view_type);

        Some(quote! {
            #(#view_fields,)*
        })
    }
}

impl ToTokens for FieldList {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.as_type());
    }
}

///
/// Field
///

#[derive(Clone, Debug, FromMeta)]
pub struct Field {
    #[darling(rename = "name")]
    pub ident: Ident,

    pub value: Value,

    #[darling(default)]
    pub default: Option<Arg>,
}

impl AsSchema for Field {
    const EMIT_SCHEMA: bool = false;

    fn schema(&self) -> TokenStream {
        let name = quote_one(&self.ident, to_str_lit);
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

impl AsType for Field {
    fn as_type(&self) -> Option<TokenStream> {
        let ident = &self.ident;
        let value = &self.value;

        Some(quote! {
            pub #ident: #value
        })
    }

    fn as_view_type(&self) -> Option<TokenStream> {
        let ident = &self.ident;
        let value_view = AsType::as_view_type(&self.value);

        Some(quote! {
            pub #ident: #value_view
        })
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.as_type());
    }
}
