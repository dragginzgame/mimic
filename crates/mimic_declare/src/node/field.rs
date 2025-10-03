use crate::prelude::*;
use std::slice::Iter;

///
/// FieldList
///

#[derive(Clone, Debug, Default, FromMeta)]
pub struct FieldList {
    #[darling(multiple, rename = "field")]
    pub fields: Vec<Field>,
}

impl FieldList {
    pub fn get(&self, ident: &Ident) -> Option<&Field> {
        self.fields.iter().find(|f| f.ident == *ident)
    }

    pub const fn len(&self) -> usize {
        self.fields.len()
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

    pub fn push(&mut self, field: Field) {
        self.fields.push(field);
    }
}

impl<'a> IntoIterator for &'a FieldList {
    type Item = &'a Field;
    type IntoIter = Iter<'a, Field>;

    fn into_iter(self) -> Self::IntoIter {
        self.fields.iter()
    }
}

impl HasSchemaPart for FieldList {
    fn schema_part(&self) -> TokenStream {
        let fields = quote_slice(&self.fields, Field::schema_part);

        quote! {
            ::mimic::schema::node::FieldList {
                fields: #fields,
            }
        }
    }
}

impl HasTypePart for FieldList {
    fn type_part(&self) -> TokenStream {
        let fields = self.fields.iter().map(HasTypePart::type_part);

        quote! {
            #(#fields,)*
        }
    }

    fn view_type_part(&self) -> TokenStream {
        let view_fields = self.fields.iter().map(HasTypePart::view_type_part);

        quote! {
            #(#view_fields,)*
        }
    }
}

///
/// Field
///

#[derive(Clone, Debug, FromMeta)]
pub struct Field {
    pub ident: Ident,

    pub value: Value,

    #[darling(default)]
    pub default: Option<Arg>,
}

impl Field {
    pub fn created_at() -> Self {
        Self {
            ident: format_ident!("created_at"),
            value: Value {
                item: Item::primitive(Primitive::Timestamp),
                ..Default::default()
            },

            default: None,
        }
    }

    pub fn updated_at() -> Self {
        Self {
            ident: format_ident!("updated_at"),
            value: Value {
                item: Item::primitive(Primitive::Timestamp),
                ..Default::default()
            },

            default: None,
        }
    }
}

impl HasSchemaPart for Field {
    fn schema_part(&self) -> TokenStream {
        let ident = quote_one(&self.ident, to_str_lit);
        let value = self.value.schema_part();
        let default = quote_option(self.default.as_ref(), Arg::schema_part);

        quote! {
            ::mimic::schema::node::Field {
                ident: #ident,
                value: #value,
                default: #default,
            }
        }
    }
}

impl HasTypePart for Field {
    fn type_part(&self) -> TokenStream {
        let ident = &self.ident;
        let value = &self.value.type_part();

        quote! {
            pub #ident: #value
        }
    }

    fn view_type_part(&self) -> TokenStream {
        let ident = &self.ident;
        let value_view = &self.value.view_type_part();

        quote! {
            pub #ident: #value_view
        }
    }
}
