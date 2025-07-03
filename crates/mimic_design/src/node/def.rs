use crate::{
    helper::{as_tokens, quote_one, quote_option, to_str_lit},
    schema::Schemable,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, LitStr};

///
/// Def
///
/// the default gets overridden after the initial darling::from_list() call
/// the schema doesn't care about the generics as they're not useful as static text
///

#[derive(Clone, Debug)]
pub struct Def {
    pub comments: Option<LitStr>,
    pub tokens: TokenStream,
    pub ident: Ident,
    pub debug: bool,
}

impl Def {
    pub fn view_ident(&self) -> Ident {
        format_ident!("{}_View", self.ident)
    }
}

impl Default for Def {
    fn default() -> Self {
        Self {
            comments: None,
            tokens: TokenStream::default(),
            ident: format_ident!("temp"),
            debug: false,
        }
    }
}

impl Schemable for Def {
    fn schema(&self) -> TokenStream {
        let comments = quote_option(self.comments.as_ref(), as_tokens);
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
