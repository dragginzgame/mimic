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
}

impl Schemable for Item {
    fn schema(&self) -> TokenStream {
        let is = quote_option(self.is.as_ref(), to_path);
        let relation = quote_option(self.relation.as_ref(), to_path);
        let selector = quote_option(self.selector.as_ref(), to_path);
        let indirect = self.indirect;

        quote! {
            ::mimic::orm::schema::node::Item{
                is: #is,
                relation: #relation,
                selector: #selector,
                indirect: #indirect,
            }
        }
    }
}

impl ToTokens for Item {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match (&self.is, &self.relation) {
            (Some(is), None) if self.indirect => quote!(Box<#is>),
            (Some(is), None) => quote!(#is),
            (None, Some(_)) => {
                quote!(::mimic::orm::base::types::Ulid)
            }
            _ => panic!("either is or relation should be set"),
        });
    }
}
