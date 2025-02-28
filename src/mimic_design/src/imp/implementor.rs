use crate::node::{Def, Trait};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// Implementor
///

pub struct Implementor<'a> {
    def: &'a Def,
    trait_: Trait,
    trait_generics: Vec<TokenStream>,
    tokens: TokenStream,
}

impl<'a> Implementor<'a> {
    pub fn new(def: &'a Def, trait_: Trait) -> Self {
        Self {
            def,
            trait_,
            trait_generics: Vec::new(),
            tokens: quote!(),
        }
    }

    // set_tokens
    pub fn set_tokens(mut self, tokens: TokenStream) -> Self {
        self.tokens = tokens;

        self
    }
}

impl ToTokens for Implementor<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // vars
        let ident = &self.def.ident;
        let inner_tokens = &self.tokens;
        let trait_ = &self.trait_;
        let trait_generics = &self.trait_generics;

        // quote
        tokens.extend(quote! {
            impl #trait_<#(#trait_generics),*> for #ident  {
                #inner_tokens
            }
        });
    }
}
