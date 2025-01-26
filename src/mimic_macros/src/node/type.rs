use crate::{
    helper::{quote_one, to_path},
    node::Args,
    traits::Schemable,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Path;

///
/// TypeValidator
///

#[derive(Clone, Debug, FromMeta)]
pub struct TypeValidator {
    pub path: Path,

    #[darling(default)]
    pub args: Args,
}

impl Schemable for TypeValidator {
    fn schema(&self) -> TokenStream {
        let path = quote_one(&self.path, to_path);
        let args = &self.args.schema();

        let q = quote! {
            ::mimic::orm::schema::node::TypeValidator {
                path: #path,
                args: #args,
            }
        };

        //  panic!("{q}");

        q
    }
}
