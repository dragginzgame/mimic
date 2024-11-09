use super::Implementor;
use crate::node::{EntityId, MacroNode, Trait};
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
