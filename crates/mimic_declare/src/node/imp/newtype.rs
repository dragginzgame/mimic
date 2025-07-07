use crate::node::Newtype;
use proc_macro2::TokenStream;
use quote::quote;

///
/// Newtype
///

pub fn tokens(node: &Newtype) -> Option<TokenStream> {
    let mut q = quote!();

    let ident = &node.def.ident;
    let item = &node.item;
    let primitive = &node.primitive.as_type();

    // From Into<primitive>
    q.extend(quote! {
        impl<T> From<T> for #ident
        where
            T: Into<#primitive>,
        {
            fn from(t: T) -> Self {
                Self(<#item as std::convert::From<#primitive>>::from(t.into()))
            }
        }
    });

    Some(q)
}
