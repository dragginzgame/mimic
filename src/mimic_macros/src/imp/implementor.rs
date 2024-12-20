use crate::node::{Def, Trait};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse2, GenericParam, Generics, WherePredicate};

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
        let impl_generics = def.generics.clone();

        Self {
            def,
            trait_,
            impl_generics,
            trait_generics: Vec::new(),
            tokens: quote!(),
        }
    }

    //
    // Method Chains
    //

    pub fn add_impl_constraint(mut self, tokens: TokenStream) -> Self {
        let constraint: WherePredicate = parse2(tokens).unwrap();
        let where_clause = self.impl_generics.make_where_clause();
        where_clause.predicates.push(constraint);

        self
    }

    pub fn add_impl_generic(mut self, tokens: TokenStream) -> Self {
        let generic_param = syn::parse2::<GenericParam>(tokens).unwrap();
        self.impl_generics.params.push(generic_param);

        self
    }

    pub fn add_trait_generic(mut self, tokens: TokenStream) -> Self {
        self.trait_generics.push(tokens);

        self
    }

    pub fn set_tokens(mut self, tokens: TokenStream) -> Self {
        self.tokens = tokens;

        self
    }
}

impl ToTokens for Implementor<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // vars
        let ident = &self.def.ident;
        let type_generics = &self.def.generics;
        let inner_tokens = &self.tokens;
        let trait_ = &self.trait_;
        let trait_generics = &self.trait_generics;

        // split
        let (impl_impl, _, impl_where) = self.impl_generics.split_for_impl();
        let (_, type_type, type_where) = type_generics.split_for_impl();

        // quote
        tokens.extend(
            quote! {
                impl #impl_impl #trait_<#(#trait_generics),*> for #ident #type_type #type_where #impl_where {
                    #inner_tokens
                }
            }
        );
    }
}
