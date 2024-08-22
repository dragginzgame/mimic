#![allow(clippy::wildcard_imports)]
// darling(default) generates these errors
#![allow(clippy::option_if_let_else)]
#![allow(clippy::manual_unwrap_or_default)]

mod helper;
mod imp;
mod node;

use crate::{
    helper::extract_comments,
    node::{Def, Node},
};
use darling::{ast::NestedMeta, Error as DarlingError, FromMeta};
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct, Visibility};

///
/// Node Macro Macros
///

macro_rules! macro_node {
    ($fn_name:ident, $node_type:ty) => {
        #[proc_macro_attribute]
        pub fn $fn_name(args: TokenStream, input: TokenStream) -> TokenStream {
            let comments = extract_comments(input.clone().into());

            match NestedMeta::parse_meta_list(args.into()) {
                Ok(args) => {
                    let item = parse_macro_input!(input as ItemStruct);

                    // validate
                    if !matches!(item.vis, Visibility::Public(_)) {
                        return TokenStream::from(
                            DarlingError::custom("expected public visibility").write_errors(),
                        );
                    }

                    // Check if the `#[debug]` attribute is present
                    let debug = item.attrs.iter().any(|attr| attr.path().is_ident("debug"));

                    // build def
                    let mut node = <$node_type>::from_list(&args).unwrap();
                    node.def = Def {
                        comments,
                        ident: item.ident.clone(),
                        generics: item.generics.clone(),
                        debug,
                    };

                    // expand tokens
                    node.expand().into()
                }
                Err(e) => TokenStream::from(DarlingError::from(e).write_errors()),
            }
        }
    };
}

// macro macros
macro_node!(canister, node::Canister);
macro_node!(constant, node::Constant);
macro_node!(entity, node::Entity);
macro_node!(enum_, node::Enum);
macro_node!(enum_hash, node::EnumHash);
macro_node!(enum_value, node::EnumValue);
macro_node!(fixture, node::Fixture);
macro_node!(map, node::Map);
macro_node!(newtype, node::Newtype);
macro_node!(permission, node::Permission);
macro_node!(primitive, node::Primitive);
macro_node!(record, node::Record);
macro_node!(role, node::Role);
macro_node!(sanitizer, node::Sanitizer);
macro_node!(store, node::Store);
macro_node!(tuple, node::Tuple);
macro_node!(validator, node::Validator);
