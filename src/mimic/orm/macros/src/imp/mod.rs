pub mod default;
pub mod display;
pub mod entity;
pub mod enum_hash;
pub mod enum_value;
pub mod filterable;
pub mod from;
pub mod inner;
pub mod num;
pub mod orderable;
pub mod primary_key;
pub mod record_filter;
pub mod record_sort;
pub mod sanitize_auto;
pub mod validate_auto;
pub mod visitable;

use crate::node::{Def, MacroNode, Trait};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse2, GenericParam, Generics, WherePredicate};

///
/// any
///
/// shared implementation
/// that can be used by a Node of any type
///

pub fn any<N: MacroNode>(node: &N, t: Trait) -> TokenStream {
    let def = node.def();

    let imp = match t {
        Trait::Path => {
            let ident_str = format!("{}", def.ident);
            let q = quote! {
                const IDENT: &'static str = #ident_str;
                const PATH: &'static str = concat!(module_path!(), "::", #ident_str);
            };

            Implementor::new(def, t).set_tokens(q).to_token_stream()
        }

        Trait::Storable => {
            let q = quote! {
                fn to_bytes(&self) -> ::std::borrow::Cow<[u8]> {
                    let serialized_data = ::mimic::orm::serialize(self).unwrap();
                    ::std::borrow::Cow::Owned(serialized_data)
                }

                fn from_bytes(bytes: ::std::borrow::Cow<[u8]>) -> Self {
                    ::mimic::orm::deserialize(&bytes).unwrap()
                }

                const BOUND: ::ic::storage::storable::Bound = ::ic::storage::storable::Bound::Unbounded;
            };

            Implementor::new(def, t).set_tokens(q).to_token_stream()
        }

        // empty implementations are generated for these traits
        Trait::Filterable
        | Trait::Orderable
        | Trait::SanitizeManual
        | Trait::SanitizeAuto
        | Trait::ValidateManual
        | Trait::ValidateAuto
        | Trait::Visitable => Implementor::new(def, t).to_token_stream(),

        _ => quote!(),
    };

    imp
}

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

impl<'a> ToTokens for Implementor<'a> {
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
