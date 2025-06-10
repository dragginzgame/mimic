use crate::helper::{as_string, quote_one, to_string};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

///
/// Def
///
/// the default gets overridden after the initial darling::from_list() call
/// the schema doesn't care about the generics as they're not useful as static text
///

#[derive(Clone, Debug)]
pub struct Def {
    pub comments: String,
    pub tokens: TokenStream,
    pub ident: Ident,
    pub debug: bool,
}

impl Default for Def {
    fn default() -> Self {
        Self {
            comments: String::default(),
            tokens: TokenStream::default(),
            ident: format_ident!("temp"),
            debug: false,
        }
    }
}

impl Def {
    pub fn schema(&self) -> TokenStream {
        let comments = quote_one(&self.comments, as_string);
        let ident = quote_one(&self.ident, to_string);

        quote! {
            ::mimic::schema::node::Def {
                module_path: module_path!().to_string(),
                comments: #comments,
                ident: #ident,
            }
        }
    }
}
