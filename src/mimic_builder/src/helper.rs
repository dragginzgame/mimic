use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::Ident;

///
/// QUOTING
///

// quote_one
pub fn quote_one<T, F>(t: &T, transform: F) -> TokenStream
where
    F: FnOnce(&T) -> TokenStream,
{
    let transformed = transform(t);
    quote!(#transformed)
}

// quote_option
pub fn quote_option<T, F>(opt: Option<&T>, transform: F) -> TokenStream
where
    F: FnOnce(&T) -> TokenStream,
{
    if let Some(v) = opt {
        let transformed = transform(v);
        quote!(Some(#transformed))
    } else {
        quote!(None)
    }
}

// quote_vec
pub fn quote_vec<T, F>(vec: &[T], transform: F) -> TokenStream
where
    F: Fn(&T) -> TokenStream,
{
    let items: Vec<TokenStream> = vec
        .iter()
        .map(transform)
        .filter(|ts| !ts.is_empty())
        .collect();

    quote! {
        vec![#(#items),*]
    }
}

///
/// TRANSFORM HELPERS
///

/// as_quote
#[expect(dead_code)]
pub fn as_quote<T: ToTokens>(t: &T) -> TokenStream {
    quote!(#t)
}

/// as_string
pub fn as_string(s: &String) -> TokenStream {
    quote!(#s.to_string())
}

/// to_string
pub fn to_string<T: ToTokens>(t: &T) -> TokenStream {
    let s = quote!(#t).to_string();

    quote!(#s.to_string())
}

/// to_path
pub fn to_path<T: ToTokens>(t: &T) -> TokenStream {
    quote! {
        <#t as ::mimic::orm::traits::Path>::path().to_string()
    }
}

///
/// DARLING HELPERS
///

// split_idents
#[allow(clippy::needless_pass_by_value)]
#[must_use]
pub fn split_idents(s: String) -> Vec<Ident> {
    s.split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(|item| format_ident!("{item}"))
        .collect()
}
