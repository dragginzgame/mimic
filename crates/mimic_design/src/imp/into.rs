use crate::{
    imp::{Imp, Implementor},
    node::{EntityId, Selector, Trait},
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// IntoTrait
///

pub struct IntoTrait {}

///
/// EntityId
///

impl Imp<EntityId> for IntoTrait {
    fn tokens(node: &EntityId, t: Trait) -> Option<TokenStream> {
        let mut tokens = quote!();

        //
        // ulid
        //

        let q = quote! {
            fn into(self) -> mimic::types::Ulid {
                self.ulid()
            }
        };

        tokens.extend(
            Implementor::new(&node.def, t)
                .set_tokens(q)
                .add_trait_generic(quote!(mimic::types::Ulid))
                .to_token_stream(),
        );

        //
        // relation
        //

        let q = quote! {
            fn into(self) -> mimic::types::Key {
                self.key()
            }
        };

        tokens.extend(
            Implementor::new(&node.def, t)
                .set_tokens(q)
                .add_trait_generic(quote!(mimic::types::Key))
                .to_token_stream(),
        );

        Some(tokens)
    }
}

///
/// Selector
///

impl Imp<Selector> for IntoTrait {
    fn tokens(node: &Selector, t: Trait) -> Option<TokenStream> {
        let target = &node.target;

        // iterate variants
        let mut inner = quote!();
        for variant in &node.variants {
            let name = &variant.name;
            let value = &variant.value;

            inner.extend(quote! {
                Self::#name => <#target as ::std::convert::From<_>>::from(#value),
            });
        }

        // match cardinality
        let q = quote! {
            fn into(self) -> #target {
                match self {
                    #inner
                }
            }
        };

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .add_trait_generic(quote!(#target))
            .to_token_stream();

        Some(tokens)
    }
}
