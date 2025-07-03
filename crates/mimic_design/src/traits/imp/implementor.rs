use crate::{node::Def, traits::Trait};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Generics;

///
/// Implementor
///

pub struct Implementor<'a> {
    def: &'a Def,
    trait_: Trait,
    impl_generics: Generics,
    trait_generics: Vec<TokenStream>,
    tokens: TokenStream,
}

impl<'a> Implementor<'a> {
    pub fn new(def: &'a Def, trait_: Trait) -> Self {
        Self {
            def,
            trait_,
            impl_generics: Generics::default(),
            trait_generics: Vec::new(),
            tokens: quote!(),
        }
    }

    //
    // Method Chains
    //

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

        // split
        let (impl_impl, _, impl_where) = self.impl_generics.split_for_impl();

        // quote
        tokens.extend(quote! {
            impl #impl_impl #trait_<#(#trait_generics),*> for #ident #impl_where {
                #inner_tokens
            }
        });
    }
}
