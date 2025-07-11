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
    fn as_type(&self) -> TokenStream {
        let fields = &self.fields;

        quote! {
            #(#fields,)*
        }
    }

    fn as_view_type(&self) -> TokenStream {
        let view_fields = self.fields.iter().map(AsType::as_view_type);

        quote! {
            #(#view_fields,)*
        }
    }

    fn view_default(&self) -> TokenStream {
        let view_defaults = self.fields.iter().map(AsType::view_default);

        quote! {
            #(#view_defaults,)*
        }
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
    pub name: Ident,
    pub value: Value,

    #[darling(default)]
    pub default: Option<Arg>,
}

impl AsSchema for Field {
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

impl AsType for Field {
    fn as_type(&self) -> TokenStream {
        let name = &self.name;
        let value = &self.value;

        quote! {
            pub #name : #value
        }
    }

    fn as_view_type(&self) -> TokenStream {
        let name = &self.name;
        let value_view = AsType::as_view_type(&self.value);

        quote! {
            pub #name: #value_view
        }
    }

    fn view_default(&self) -> TokenStream {
        let name = &self.name;
        let value_default = AsType::view_default(&self.value);

        quote! {
            #name: #value_default
        }
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.as_type());
    }
}
