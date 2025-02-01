use crate::{
    helper::{quote_option, to_path},
    traits::Schemable,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Path;

///
/// Item
///

#[derive(Clone, Debug, FromMeta)]
pub struct Item {
    #[darling(default)]
    pub is: Option<Path>,

    #[darling(default, rename = "rel")]
    pub relation: Option<Path>,

    #[darling(default)]
    pub selector: Option<Path>,

    #[darling(default)]
    pub indirect: bool,

    #[darling(default)]
    pub todo: bool,
}

impl Schemable for Item {
    fn schema(&self) -> TokenStream {
        let is = quote_option(self.is.as_ref(), to_path);
        let relation = quote_option(self.relation.as_ref(), to_path);
        let selector = quote_option(self.selector.as_ref(), to_path);
        let indirect = self.indirect;
        let todo = self.todo;

        quote! {
            ::mimic::orm::schema::node::Item{
                is: #is,
                relation: #relation,
                selector: #selector,
                indirect: #indirect,
                todo: #todo,
            }
        }
    }
}

impl ToTokens for Item {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let q = if self.todo {
            // todo we turn to i32
            quote!(::mimic::orm::base::types::I32)
        } else {
            match (&self.is, &self.relation) {
                (Some(is), None) if self.indirect => quote!(Box<#is>),
                (Some(is), None) => quote!(#is),

                // relation
                (None, Some(_)) => {
                    quote!(::mimic::orm::base::types::Ulid)
                }
                _ => panic!("either is or relation should be set"),
            }
        };

        tokens.extend(q)
    }
}
