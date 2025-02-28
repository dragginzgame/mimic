use super::Implementor;
use crate::node::{Arg, Entity, FieldList, MacroNode, Newtype, Record, Trait};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

// entity
pub fn entity(node: &Entity, t: Trait) -> Option<TokenStream> {
    let q = field_list(&node.fields);
    let tokens = Implementor::new(node.def(), t)
        .set_tokens(q)
        .to_token_stream();

    Some(tokens)
}

// record
pub fn record(node: &Record, t: Trait) -> Option<TokenStream> {
    let q = field_list(&node.fields);
    let tokens = Implementor::new(node.def(), t)
        .set_tokens(q)
        .to_token_stream();

    Some(tokens)
}

// field_list
fn field_list(node: &FieldList) -> TokenStream {
    // inner
    let mut inner = quote!();
    for field in &node.fields {
        let name = &field.name;

        if let Some(default) = &field.default {
            let arg = format_default(default);

            inner.extend(quote! {
                #name: #arg,
            });
        } else {
            inner.extend(quote! {
                #name: Default::default(),
            });
        }
    }

    // quote
    quote! {
        fn default() -> Self {
            Self {
                #inner
            }
        }
    }
}

// newtype
pub fn newtype(node: &Newtype, t: Trait) -> Option<TokenStream> {
    let inner = match &node.default {
        Some(arg) => format_default(arg),
        None => panic!("default impl but no default"),
    };

    // quote
    let q = quote! {
        fn default() -> Self {
            Self(#inner)
        }
    };

    let tokens = Implementor::new(node.def(), t)
        .set_tokens(q)
        .to_token_stream();

    Some(tokens)
}

// format_default
// not 100% sure NumCast will always work here, may need some extra checks
fn format_default(arg: &Arg) -> TokenStream {
    match arg {
        Arg::Path(path) => quote!(#path().into()),
        Arg::Bool(v) => quote!(#v.into()),
        Arg::Char(v) => quote!(#v.into()),
        Arg::Number(v) => quote!(::mimic::orm::traits::NumCast::from(#v).expect("number is valid")),
        Arg::String(v) => quote!(#v.into()),
    }
}
