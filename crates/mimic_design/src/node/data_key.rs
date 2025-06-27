use crate::helper::{quote_one, quote_option, to_path, to_str_lit};
use darling::FromMeta;
use mimic::schema::traits::Schemable;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Path};

///
/// DataKey
///

#[derive(Debug, FromMeta)]
pub struct DataKey {
    pub entity: Path,

    #[darling(default)]
    pub field: Option<Ident>,
}

impl Schemable for DataKey {
    fn schema(&self) -> TokenStream {
        let entity = quote_one(&self.entity, to_path);
        let field = quote_option(self.field.as_ref(), to_str_lit);

        quote! {
            ::mimic::schema::node::DataKey {
                entity: #entity,
                field: #field,
            }
        }
    }
}
