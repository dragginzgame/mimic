use proc_macro2::{TokenStream, TokenTree};
use quote::ToTokens;
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
pub fn quote_option<T, F>(opt: &Option<T>, transform: F) -> TokenStream
where
    F: FnOnce(&T) -> TokenStream,
{
    match opt {
        Some(v) => {
            let transformed = transform(v);
            quote!(Some(#transformed))
        }
        None => quote!(None),
    }
}

// quote_vec
pub fn quote_vec<T, F>(vec: &[T], transform: F) -> TokenStream
where
    F: Fn(&T) -> TokenStream,
{
    let items: Vec<TokenStream> = vec.iter().map(transform).collect();
    quote! {
        vec![#(#items),*]
    }
}

///
/// TRANSFORM HELPERS
///

/// as_self
pub fn as_self<T: ToTokens>(t: &T) -> TokenStream {
    quote!(#t)
}

/// as_string
pub fn as_string<T: ToTokens>(t: &T) -> TokenStream {
    quote!(#t.to_string())
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
/// READING`1
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

// extract_comments
#[must_use]
pub fn extract_comments(input: TokenStream) -> String {
    let mut comments = Vec::new();

    for token in input {
        match token {
            TokenTree::Group(group) => {
                if group.delimiter() == proc_macro2::Delimiter::Bracket {
                    let mut inner_tokens = group.stream().into_iter();
                    if let (
                        Some(TokenTree::Ident(ident)),
                        Some(TokenTree::Punct(punct)),
                        Some(TokenTree::Literal(lit)),
                    ) = (
                        inner_tokens.next(),
                        inner_tokens.next(),
                        inner_tokens.next(),
                    ) {
                        if ident == "doc" && punct.as_char() == '=' {
                            let comment = lit.to_string().trim_matches('"').to_string();
                            comments.push(comment);
                        }
                    }
                }
            }
            _ => continue,
        }
    }

    let comments = comments.join("\n");

    comments
        .trim_matches(|c| c == ' ' || c == '\n' || c == '"')
        .to_string()
}
