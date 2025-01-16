use crate::{
    helper::{quote_vec, split_idents, to_string},
    traits::Schemable,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

///
/// Index
///

#[derive(Debug, FromMeta)]
pub struct Index {
    #[darling(default, map = "split_idents")]
    pub fields: Vec<Ident>,

    #[darling(default)]
    pub unique: bool,
}

impl Schemable for Index {
    fn schema(&self) -> TokenStream {
        let fields = quote_vec(&self.fields, to_string);
        let unique = &self.unique;

        quote! {
            ::mimic::orm::schema::node::Index {
                fields: #fields,
                unique: #unique,
            }
        }
    }
}
