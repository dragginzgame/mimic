use crate::{
    helper::{as_tokens, quote_one, quote_option, to_str_lit},
    traits::HasSchemaPart,
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

#[derive(Debug)]
pub struct Def {
    pub comments: Option<LitStr>,
    pub ident: Ident,
}

impl Default for Def {
    fn default() -> Self {
        Self {
            comments: None,
            ident: format_ident!("_"),
        }
    }
}

impl HasSchemaPart for Def {
    fn schema_part(&self) -> TokenStream {
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
