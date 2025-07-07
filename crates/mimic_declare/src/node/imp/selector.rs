use crate::node::Selector;
use proc_macro2::TokenStream;
use quote::quote;

///
/// Selector
///

pub fn tokens(node: &Selector) -> Option<TokenStream> {
    let ident = &node.def.ident;
    let target = &node.target;

    // build match arms for each variant
    let match_arms = node.variants.iter().map(|variant| {
        let name = &variant.name;
        let value = &variant.value;

        quote! {
            Self::#name => <#target as ::std::convert::From<_>>::from(#value),
        }
    });

    // Into
    let q = quote! {
        impl Into<#target> for #ident {
            fn into(self) -> #target {
                match self {
                    #(#match_arms)*
                }
            }
        }
    };

    Some(q)
}
