use proc_macro2::{TokenStream, TokenTree};
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
pub fn quote_option<T, F>(opt: &Option<T>, transform: F) -> TokenStream
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
/// READING
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
                            let comment = lit.to_string();
                            // Remove the outermost quotes and the first space of each line
                            let cleaned_comment = clean_comment(&comment);

                            comments.push(cleaned_comment);
                        }
                    }
                }
            }
            _ => continue,
        }
    }

    let comments = comments
        .into_iter()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
        .replace(r#"\""#, r#"""#);

    comments
}

/// Trims the outermost quotes, if they are unescaped, and removes the first space of each line
fn clean_comment(literal: &str) -> String {
    let mut chars = literal.chars().peekable();
    let mut result = String::new();

    // Check if the first character is an unescaped quote
    if chars.peek() == Some(&'"') {
        chars.next(); // Skip the starting quote
    }

    while let Some(c) = chars.next() {
        if c == '"' && chars.peek().is_none() {
            // Skip the ending quote if it's the last character
            break;
        }
        result.push(c);
    }

    // Split the string into lines and remove the first space of each line if it exists
    result
        .lines()
        .map(|line| line.strip_prefix(' ').unwrap_or(line)) // Remove the first space if present
        .collect::<Vec<_>>()
        .join("\n")
}
