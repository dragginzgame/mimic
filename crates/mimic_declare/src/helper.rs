use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Ident, Path};

///
/// IDENT
///

pub fn format_view_ident<T: ToString>(ty: T) -> Ident {
    let base = ty.to_string();

    // Try parsing as a Rust path (like `types::bytes::Utf8`)
    if let Ok(path) = syn::parse_str::<Path>(&base) {
        if let Some(last) = path.segments.last() {
            return format_ident!("{}_View", last.ident);
        }
    }

    // Fallback: treat the whole thing as one identifier (only safe if no `::`)
    format_ident!("{}_View", base.replace("::", "_"))
}

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

// quote_slice
pub fn quote_slice<T, F>(vec: &[T], transform: F) -> TokenStream
where
    F: Fn(&T) -> TokenStream,
{
    let items: Vec<TokenStream> = vec
        .iter()
        .map(transform)
        .filter(|ts| !ts.is_empty())
        .collect();

    quote! {
        &[#(#items),*]
    }
}

///
/// TRANSFORM HELPERS
///

/// as_tokens
/// passes through without any change, ie. comments
pub fn as_tokens<T: ToTokens>(t: &T) -> TokenStream {
    quote!(#t)
}

/// to_str_lit
pub fn to_str_lit<T: ToTokens>(t: &T) -> TokenStream {
    let s = quote!(#t).to_string();

    quote!(#s)
}

/// to_path
pub fn to_path<T: ToTokens>(t: &T) -> TokenStream {
    quote! {
        <#t as ::mimic::core::traits::Path>::PATH
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
