use crate::{
    helper::{as_string, quote_option, quote_vec},
    node::ArgNumber,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use schema::Schemable;
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
            ::mimic::schema::node::Guide {
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
        let name = quote_option(&self.name, as_string);
        let value = &self.value.schema();

        quote! {
            ::mimic::schema::node::GuideEntry {
                name: #name,
                value: #value,
            }
        }
    }
}
