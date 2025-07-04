#![allow(clippy::wildcard_imports)]
// darling(default) generates these errors
#![allow(clippy::option_if_let_else)]
#![allow(clippy::manual_unwrap_or_default)]

mod helper;
mod imp;
mod node;
mod schema;
mod traits;

use crate::node::{Def, Node};
use darling::{Error as DarlingError, FromMeta, ast::NestedMeta};
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use quote::quote;
use syn::{ItemStruct, Visibility, parse_macro_input};

///
/// Node Macro Macros
///

macro_rules! macro_node {
    ($fn_name:ident, $node_type:ty) => {
        #[proc_macro_attribute]
        pub fn $fn_name(
            args: proc_macro::TokenStream,
            input: proc_macro::TokenStream,
        ) -> proc_macro::TokenStream {
            let comments = extract_comments(input.clone().into());

            match NestedMeta::parse_meta_list(args.into()) {
                Ok(args) => {
                    let item = parse_macro_input!(input as ItemStruct);

                    // validate
                    if !matches!(item.vis, Visibility::Public(_)) {
                        return proc_macro::TokenStream::from(
                            DarlingError::custom("expected public visibility").write_errors(),
                        );
                    }

                    // Save the struct's code as tokens
                    let tokens = quote! { #item };

                    // Check if the `#[debug]` attribute is present
                    let debug = item.attrs.iter().any(|attr| attr.path().is_ident("debug"));

                    // build def
                    let mut node = <$node_type>::from_list(&args).unwrap();
                    node.def = Def {
                        comments,
                        tokens,
                        ident: item.ident.clone(),
                        debug,
                    };

                    // expand tokens
                    node.expand().into()
                }
                Err(e) => proc_macro::TokenStream::from(DarlingError::from(e).write_errors()),
            }
        }
    };
}

// macro macros
macro_node!(canister, node::Canister);
macro_node!(constant, node::Constant);
macro_node!(entity, node::Entity);
macro_node!(entity_id, node::EntityId);
macro_node!(enum_, node::Enum);
macro_node!(enum_value, node::EnumValue);
macro_node!(list, node::List);
macro_node!(map, node::Map);
macro_node!(newtype, node::Newtype);
macro_node!(record, node::Record);
macro_node!(selector, node::Selector);
macro_node!(set, node::Set);
macro_node!(store, node::Store);
macro_node!(tuple, node::Tuple);
macro_node!(validator, node::Validator);

///
/// Helper Functions
///

// extract_comments
#[must_use]
fn extract_comments(input: TokenStream) -> String {
    let mut comments = Vec::new();

    for token in input {
        if let TokenTree::Group(group) = token {
            if group.delimiter() == Delimiter::Bracket {
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
    }

    comments
        .into_iter()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
        .replace(r#"\""#, r#"""#)
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
