use crate::{
    helper::{quote_one, quote_slice, to_path},
    node::Args,
    traits::HasSchemaPart,
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

    #[darling(multiple, rename = "sanitizer")]
    pub sanitizers: Vec<TypeSanitizer>,

    #[darling(multiple, rename = "validator")]
    pub validators: Vec<TypeValidator>,
}

impl HasSchemaPart for Type {
    fn schema_part(&self) -> TokenStream {
        let sanitizers = quote_slice(&self.sanitizers, TypeSanitizer::schema_part);
        let validators = quote_slice(&self.validators, TypeValidator::schema_part);
        let todo = self.todo;

        quote! {
            ::mimic::schema::node::Type {
                sanitizers: #sanitizers,
                validators: #validators,
                todo: #todo,
            }
        }
    }
}

///
/// TypeSanitizer
///

#[derive(Clone, Debug, FromMeta)]
pub struct TypeSanitizer {
    pub path: Path,

    #[darling(default)]
    pub args: Args,
}

impl HasSchemaPart for TypeSanitizer {
    fn schema_part(&self) -> TokenStream {
        let path = quote_one(&self.path, to_path);
        let args = &self.args.schema_part();

        quote! {
            ::mimic::schema::node::TypeSanitizer {
                path: #path,
                args: #args,
            }
        }
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

impl TypeValidator {
    pub fn quote_constructor(&self) -> TokenStream {
        let path = &self.path;
        let args = &self.args;

        if args.is_empty() {
            quote! { #path::default() }
        } else {
            quote! { #path::new(#(#args),*) }
        }
    }
}

impl HasSchemaPart for TypeValidator {
    fn schema_part(&self) -> TokenStream {
        let path = quote_one(&self.path, to_path);
        let args = &self.args.schema_part();

        quote! {
            ::mimic::schema::node::TypeValidator {
                path: #path,
                args: #args,
            }
        }
    }
}
