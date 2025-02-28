use super::Implementor;
use crate::node::{EntityId, MacroNode, Selector, Trait};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

//
// ENTITY_ID
//

pub fn entity_id(node: &EntityId, t: Trait) -> TokenStream {
    let mut q = quote!();

    q.extend(entity_id_ulid(node, t));

    q
}

pub fn entity_id_ulid(node: &EntityId, t: Trait) -> TokenStream {
    let imp = Implementor::new(node.def(), t);

    // match cardinality
    let q = quote! {
        fn into(self) -> mimic::orm::base::types::Ulid {
            self.ulid()
        }
    };

    imp.set_tokens(q)
        .add_trait_generic(quote!(mimic::orm::base::types::Ulid))
        .to_token_stream()
}

//
// SELECTOR
//

pub fn selector(node: &Selector, t: Trait) -> TokenStream {
    let imp = Implementor::new(node.def(), t);
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

    imp.set_tokens(q)
        .add_trait_generic(quote!(#target))
        .to_token_stream()
}
