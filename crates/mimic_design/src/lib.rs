#![allow(clippy::wildcard_imports)]

mod helper;
mod node;
mod schema;
mod traits;

use crate::node::{Def, Node};
use darling::{Error as DarlingError, FromMeta, ast::NestedMeta};
use proc_macro2::Span;
use quote::quote;
use syn::{Attribute, ItemStruct, LitStr, Visibility, parse_macro_input};

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
            match NestedMeta::parse_meta_list(args.into()) {
                Ok(args) => {
                    let item = parse_macro_input!(input as ItemStruct);
                    let comments = extract_comments(&item.attrs);

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
fn extract_comments(attrs: &[Attribute]) -> Option<LitStr> {
    let lines: Vec<String> = attrs
        .iter()
        .filter_map(|attr| match &attr.meta {
            syn::Meta::NameValue(meta) if meta.path.is_ident("doc") => {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit_str),
                    ..
                }) = &meta.value
                {
                    let value = lit_str.value();
                    Some(value.strip_prefix(' ').unwrap_or(&value).to_string())
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect();

    let cleaned = lines.join("\n").trim_matches('\n').to_string();

    if cleaned.is_empty() {
        None
    } else {
        Some(LitStr::new(&cleaned, Span::call_site()))
    }
}
