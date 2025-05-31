use crate::{
    helper::{quote_one, quote_option, quote_vec, to_path},
    node::TypeValidator,
    traits::Schemable,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
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

    #[darling(multiple, rename = "validator")]
    pub validators: Vec<TypeValidator>,

    #[darling(default)]
    pub indirect: bool,

    #[darling(default)]
    pub todo: bool,
}

impl Item {
    pub const fn is_relation(&self) -> bool {
        self.relation.is_some()
    }

    pub fn quoted_path(&self) -> TokenStream {
        match (&self.is, &self.relation) {
            (Some(is), None) => quote!(#is),
            (None, Some(_)) => quote!(::mimic::types::prim::Relation),
            (None, None) => quote!(::mimic::types::prim::Unit),
            _ => panic!("cannot set both is and relation"),
        }
    }
}

impl Schemable for Item {
    fn schema(&self) -> TokenStream {
        let path = quote_one(&self.quoted_path(), to_path);
        let relation = quote_option(self.relation.as_ref(), to_path);
        let selector = quote_option(self.selector.as_ref(), to_path);
        let validators = quote_vec(&self.validators, TypeValidator::schema);
        let indirect = self.indirect;
        let todo = self.todo;

        quote! {
            ::mimic::schema::node::Item{
                path: #path,
                relation: #relation,
                selector: #selector,
                validators: #validators,
                indirect: #indirect,
                todo: #todo,
            }
        }
    }
}

impl ToTokens for Item {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let path = self.quoted_path();
        let q = if self.indirect {
            quote!(Box<#path>)
        } else {
            quote!(#path)
        };

        tokens.extend(q);
    }
}
