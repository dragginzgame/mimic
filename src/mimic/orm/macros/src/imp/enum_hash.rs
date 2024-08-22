use crate::{
    imp::Implementor,
    node::{EnumHash, MacroNode, Trait},
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::hash::{DefaultHasher, Hash, Hasher};

///
/// ENUM_HASH
///

// enum_hash
pub fn enum_hash(node: &EnumHash, t: Trait) -> TokenStream {
    let mut to_inner = quote!();
    let mut try_from_inner = quote!();
    let ident = &node.def.ident;

    // iterate keys
    for key in &node.keys {
        let digest = format!("{ident}::{key}");
        let value = compute_hash(&digest);

        to_inner.extend(quote! {
            Self::#key => #value,
        });
        try_from_inner.extend(quote! {
            #value => Ok(Self::#key),
        });
    }

    // quote
    let q = quote! {
        fn to_hash(&self) -> u64 {
            match self {
                #to_inner
            }
        }

        fn try_from_hash(key: u64) -> Result<Self, ::mimic::orm::Error> {
            match key {
                #try_from_inner
                _ => Err(::mimic::orm::Error::InvalidEnumHash{ key }),
            }
        }
    };

    Implementor::new(node.def(), t)
        .set_tokens(q)
        .to_token_stream()
}

// compute_hash
fn compute_hash(item: &str) -> u64 {
    let mut s = DefaultHasher::new();
    item.hash(&mut s);

    s.finish()
}
