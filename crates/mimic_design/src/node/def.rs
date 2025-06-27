use crate::helper::{quote_one, to_str_lit};
use mimic::schema::traits::Schemable;
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
            comments: String::new(),
            tokens: TokenStream::default(),
            ident: format_ident!("temp"),
            debug: false,
        }
    }
}

impl Schemable for Def {
    fn schema(&self) -> TokenStream {
        let comments = quote_one(&self.comments, to_str_lit);
        let ident = quote_one(&self.ident, to_str_lit);

        quote! {
            ::mimic::schema::node::Def {
                module_path: module_path!(),
                comments: #comments,
                ident: #ident,
            }
        }
    }
}
