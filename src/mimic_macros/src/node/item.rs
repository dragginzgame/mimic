use crate::{
    helper::{quote_option, to_path},
    traits::Schemable,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_str, Path};

///
/// Item
///

pub static PRIM_ULID: &str = "mimic::orm::base::types::Ulid";

#[derive(Clone, Debug, FromMeta)]
pub struct Item {
    #[darling(default)]
    pub indirect: bool,

    #[darling(default)]
    pub is: Option<Path>,

    #[darling(default, rename = "rel")]
    pub relation: Option<Path>,
}

impl Item {
    // is_relation
    pub const fn is_relation(&self) -> bool {
        self.relation.is_some()
    }
}

impl Schemable for Item {
    fn schema(&self) -> TokenStream {
        let indirect = self.indirect;
        let is = quote_option(&self.is, to_path);
        let relation = quote_option(&self.relation, to_path);

        quote! {
            ::mimic::orm::schema::node::Item{
                indirect: #indirect,
                is: #is,
                relation: #relation,
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
                let ty: Path = parse_str(PRIM_ULID).unwrap();

                quote!(#ty)
            }
            _ => panic!("either is or relation should be set"),
        });
    }
}
