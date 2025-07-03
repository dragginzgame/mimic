use crate::node::Map;
use proc_macro2::TokenStream;
use quote::quote;

///
/// Map
///

pub fn tokens(node: &Map) -> Option<TokenStream> {
    let ident = &node.def.ident;
    let key = &node.key;
    let value = &node.value;

    // from Vec<K, V>
    let q = quote! {
        impl<K, V> From<Vec<(K, V)>> for #ident
        where
            K: Into<#key>,
            V: Into<#value>,
        {
            fn from(entries: Vec<(K, V)>) -> Self {
                Self(
                    entries
                        .into_iter()
                        .map(|(k, v)| (k.into(), v.into()))
                        .collect()
                )
            }
        }
    };

    Some(q)
}
