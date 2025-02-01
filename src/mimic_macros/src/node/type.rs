use crate::{
    helper::{quote_one, quote_vec, to_path},
    node::Args,
    traits::Schemable,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Path;

///
/// Type
///

#[derive(Clone, Debug, Default, FromMeta)]
pub struct Type {
    #[darling(default)]
    pub todo: bool,

    #[darling(multiple, rename = "validator")]
    pub validators: Vec<TypeValidator>,
}

impl Schemable for Type {
    fn schema(&self) -> TokenStream {
        let validators = quote_vec(&self.validators, TypeValidator::schema);
        let todo = self.todo;

        let q = quote! {
            ::mimic::orm::schema::node::Type {
                validators: #validators,
                todo: #todo,
            }
        };

        q
    }
}

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

        q
    }
}
