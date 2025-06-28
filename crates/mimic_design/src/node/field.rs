use crate::{
    helper::{quote_one, quote_option, to_str_lit},
    node::{Arg, Value},
    schema::{Cardinality, Schemable},
};
use darling::FromMeta;
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
