use crate::{
    node::Def,
    traits::{HasSchema, HasTraits, HasType, HasViewTypes},
};
use proc_macro2::TokenStream;
use quote::quote;

///
/// HasDef
///

pub trait HasDef {
    fn def(&self) -> &Def;
}

///
/// TraitTokens
///
/// Result of trait resolution — combines derived traits and manual impls.
///

pub struct TraitTokens {
    pub derive: TokenStream,
    pub impls: TokenStream,
}

///
/// HasMacro
///
/// High-level entrypoint for procedural code generation.
/// Coordinates schema emission, type emission, trait impls, and view generation.
///

pub trait HasMacro: HasSchema + HasTraits + HasType + HasViewTypes {
    /// Generate all Rust tokens for this node: schema consts, derives, impls, and view structs.
    fn all_tokens(&self) -> TokenStream {
        let TraitTokens { derive, impls } = self.resolve_trait_tokens();
        let schema = self.schema_tokens();
        let type_part = self.type_part();
        let view_parts = self.view_parts();

        quote! {
            // SCHEMA CONSTANT
            #schema

            // MAIN TYPE
            #derive
            #type_part

            // IMPLEMENTATIONS
            #impls

            // VIEW TYPES (Edit, Filter, etc.)
            #view_parts
        }
    }

    /// Resolve all derive + impl traits for this node, returning combined code.
    fn resolve_trait_tokens(&self) -> TraitTokens {
        let mut derive_traits = Vec::new();
        let mut attrs = Vec::new();
        let mut impls = TokenStream::new();

        for tr in self.traits() {
            // Each trait can either have an explicit map or fallback to default.
            let strat = self.map_trait(tr).or_else(|| self.default_strategy(tr));
            let attr = self.map_attribute(tr);

            if let Some(strategy) = strat {
                if let Some(ts) = strategy.imp {
                    impls.extend(ts);
                }

                if let Some(derive_tr) = strategy.derive
                    && let Some(path) = derive_tr.derive_path()
                {
                    derive_traits.push(path);
                }
            } else if let Some(path) = tr.derive_path() {
                derive_traits.push(path);
            }

            if let Some(attr_tokens) = attr {
                attrs.push(attr_tokens);
            }
        }

        let mut derive = if derive_traits.is_empty() {
            quote!()
        } else {
            quote!(#[derive(#(#derive_traits),*)])
        };

        derive.extend(attrs);

        TraitTokens { derive, impls }
    }
}

/// Blanket implementation so any node that satisfies the constraints
/// automatically gets full macro generation.
impl<T> HasMacro for T where T: HasDef + HasSchema + HasTraits + HasType + HasViewTypes {}
