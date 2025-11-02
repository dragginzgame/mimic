use crate::traits::HasDef;
use mimic_common::case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::Ident;

///
/// HasSchema
///
/// Anything that can emit a schema constant.
///

pub trait HasSchema: HasSchemaPart + HasDef {
    /// The kind of schema node this represents (Entity, Enum, etc.)
    fn schema_node_kind() -> SchemaNodeKind {
        unreachable!("SchemaNodeKind must be defined by each node type")
    }

    /// The uppercase snake-case constant name used in the generated schema file.
    fn schema_const(&self) -> Ident {
        let ident_s = self.def().ident().to_string().to_case(Case::UpperSnake);
        format_ident!("{ident_s}_CONST")
    }

    /// Emits the full schema constant + registration constructor.
    fn schema_tokens(&self) -> TokenStream {
        let schema_expr = self.schema_part();
        if schema_expr.is_empty() {
            return quote!();
        }

        let const_var = self.schema_const();
        let kind = Self::schema_node_kind();

        quote! {
            const #const_var: ::mimic::schema::node::#kind = #schema_expr;

            #[cfg(not(target_arch = "wasm32"))]
            #[::mimic::export::ctor::ctor(anonymous, crate_path = ::mimic::export::ctor)]
            fn __ctor() {
                ::mimic::schema::build::schema_write().insert_node(
                    ::mimic::schema::node::SchemaNode::#kind(#const_var)
                );
            }
        }
    }
}

#[derive(Debug)]
pub enum SchemaNodeKind {
    Canister,
    Entity,
    Enum,
    List,
    Map,
    Newtype,
    Record,
    Sanitizer,
    Selector,
    Set,
    Store,
    Tuple,
    Validator,
}

impl ToTokens for SchemaNodeKind {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        format_ident!("{self:?}").to_tokens(tokens);
    }
}

///
/// HasSchemaPart
///
/// Low-level helper for schema fragments.
///

pub trait HasSchemaPart {
    fn schema_part(&self) -> TokenStream {
        quote!()
    }
}
