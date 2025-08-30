use crate::{
    imp::{Imp, Implementor, Trait, TraitStrategy},
    node::Entity,
    traits::HasIdent,
};
use mimic_common::utils::case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};

///
/// InherentTrait
///

pub struct InherentTrait {}

///
/// Entity
///

impl Imp<Entity> for InherentTrait {
    fn strategy(node: &Entity) -> Option<TraitStrategy> {
        // emit typed field consts
        let field_consts: Vec<TokenStream> = node
            .fields
            .iter()
            .map(|f| {
                let constant = &f.ident.to_string().to_case(Case::Constant);
                let ident = format_ident!("{constant}");
                let name_str = f.ident.to_string();

                // right now just strings, could be your Field<Self, T> later
                quote! {
                    pub const #ident: &str = #name_str;
                }
            })
            .collect();

        let tokens = quote! {
            #(#field_consts)*
        };

        // IMPORTANT: pass Trait::Inherent so Implementor will do `impl Entity { … }`
        let tokens = Implementor::new(node.ident(), Trait::Inherent)
            .set_tokens(tokens)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}
