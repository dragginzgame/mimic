use crate::node::List;
use proc_macro2::TokenStream;
use quote::quote;

///
/// List
///

pub fn tokens(node: &List) -> Option<TokenStream> {
    let ident = &node.def.ident;
    let item = &node.item;

    let tokens = quote! {
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

    Some(tokens)
}
