use crate::schema_traits::Trait;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{GenericParam, Generics, Ident, WherePredicate, parse2};

///
/// Implementor
///

pub struct Implementor {
    ident: Ident,
    trait_: Trait,
    impl_generics: Generics,
    trait_generics: Vec<TokenStream>,
    tokens: TokenStream,
}

impl Implementor {
    pub fn new(ident: Ident, trait_: Trait) -> Self {
        Self {
            ident,
            trait_,
            impl_generics: Generics::default(),
            trait_generics: Vec::new(),
            tokens: quote!(),
        }
    }

    //
    // Method Chains
    //

    pub fn add_impl_constraint(mut self, tokens: TokenStream) -> Self {
        let constraint: WherePredicate = parse2(tokens).expect("where predicate parses");
        let where_clause = self.impl_generics.make_where_clause();
        where_clause.predicates.push(constraint);

        self
    }

    pub fn add_impl_generic(mut self, tokens: TokenStream) -> Self {
        let generic_param = syn::parse2::<GenericParam>(tokens).expect("generic param parses");
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

    // impl_header
    fn impl_header(&self) -> TokenStream {
        let ident = &self.ident;
        let trait_ = &self.trait_;
        let trait_generics = &self.trait_generics;

        // split generics like: impl<'a, T> … where …
        let (impl_impl, _, impl_where) = self.impl_generics.split_for_impl();

        match trait_ {
            Trait::Inherent => {
                quote! {
                    impl #impl_impl #ident #impl_where
                }
            }
            _ => {
                quote! {
                    impl #impl_impl #trait_<#(#trait_generics),*> for #ident #impl_where
                }
            }
        }
    }
}

impl ToTokens for Implementor {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let header = self.impl_header();
        let inner_tokens = &self.tokens;

        tokens.extend(quote! {
            #header {
                #inner_tokens
            }
        });
    }
}
