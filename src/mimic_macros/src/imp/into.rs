use super::Implementor;
use crate::node::{EntityKey, MacroNode, Trait};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

// entity_key
pub fn entity_key(node: &EntityKey, t: Trait) -> TokenStream {
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
