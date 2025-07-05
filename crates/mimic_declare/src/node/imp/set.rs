use crate::node::Set;
use proc_macro2::TokenStream;
use quote::quote;

///
/// Set
///

pub fn tokens(node: &Set) -> Option<TokenStream> {
    let ident = &node.def.ident;
    let item = &node.item;

    let q = quote! {
        impl<I> From<Vec<I>> for #ident
        where
            I: Into<#item>,
        {
            fn from(entries: Vec<I>) -> Self {
                Self(
                    entries
                        .into_iter()
                        .map(Into::into)
                        .collect()
                )
            }
        }
    };

    Some(q)
}
