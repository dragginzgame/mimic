use crate::{
    helper::{quote_option, quote_vec, to_string},
    node::ArgNumber,
};
use darling::FromMeta;
use orm_schema::Schemable;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Lit;

///
/// Guide
///

#[derive(Debug, FromMeta)]
pub struct Guide {
    #[darling(multiple, rename = "entry")]
    pub entries: Vec<GuideEntry>,
}

impl Schemable for Guide {
    fn schema(&self) -> TokenStream {
        let entries = quote_vec(&self.entries, GuideEntry::schema);

        quote! {
            ::mimic::orm::schema::node::Guide {
                entries: #entries,
            }
        }
    }
}

///
/// GuideEntry
///

#[derive(Debug, FromMeta)]
pub struct GuideEntry {
    #[darling(default)]
    pub name: Option<Lit>,

    pub value: ArgNumber,
}

impl Schemable for GuideEntry {
    fn schema(&self) -> TokenStream {
        // Lit types are automatically given quotes
        let name = quote_option(&self.name, to_string);
        let value = &self.value.schema();

        quote! {
            ::mimic::orm::schema::node::GuideEntry {
                name: #name,
                value: #value,
            }
        }
    }
}