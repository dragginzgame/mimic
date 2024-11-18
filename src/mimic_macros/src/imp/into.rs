use super::Implementor;
use crate::node::{EntityId, MacroNode, Selector, Trait};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

// entity_id
pub fn entity_id(node: &EntityId, t: Trait) -> TokenStream {
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

// selector
pub fn selector(node: &Selector, t: Trait) -> TokenStream {
    let imp = Implementor::new(node.def(), t);
    let target = &node.target;

    // iterate variants
    let mut inner = quote!();
    for variant in &node.variants {
        let name = &variant.name;
        let value = &variant.value;

        inner.extend(quote! {
            Self::#name => #target::from(#value),
        });
    }

    // match cardinality
    let q = quote! {
        fn into(self) -> #target {
            #inner
        }
    };

    imp.set_tokens(q)
        .add_trait_generic(quote!(#target))
        .to_token_stream()
}
