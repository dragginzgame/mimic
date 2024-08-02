use crate::helper::{quote_one, quote_vec, split_idents, to_path, to_string};
use darling::FromMeta;
use orm_schema::Schemable;
use proc_macro2::TokenStream;
use syn::{Ident, Path};
use quote::quote;

///
/// SortKey
///

#[derive(Debug, FromMeta)]
pub struct SortKey {
    pub entity: Path,

    #[darling(default, map = "split_idents")]
    pub fields: Vec<Ident>,
}

impl Schemable for SortKey {
    fn schema(&self) -> TokenStream {
        let entity = quote_one(&self.entity, to_path);
        let fields = quote_vec(&self.fields, to_string);

        quote! {
            ::mimic::orm::schema::node::SortKey {
                entity: #entity,
                fields: #fields,
            }
        }
    }
}
