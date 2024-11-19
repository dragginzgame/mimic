use super::Implementor;
use crate::node::{Newtype, PrimitiveGroup, Trait};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

// newtype
pub fn newtype(node: &Newtype, t: Trait) -> TokenStream {
    // Generate inner logic
    let inner = if node.validators.is_empty() {
        quote!()
    } else {
        // sanitize function name
        let sanitize_fn = match node.primitive {
            Some(prim) => match prim.group() {
                PrimitiveGroup::Decimal | PrimitiveGroup::Integer => quote! { sanitize_number },
                PrimitiveGroup::String => quote! { sanitize_string },

                _ => panic!("sanitizer error - invalid primitive group"),
            },
            None => panic!("sanitizer error - no primitive"),
        };

        // Generate rules
        let rules = node.sanitizers.iter().map(|san| {
            let path = &san.path;
            let args = &san.args;

            let constructor = match args.len() {
                0 => quote! { #path::default() },
                _ => quote! { #path::new(#(#args),*) },
            };

            quote! {
                self.0 = #constructor.#sanitize_fn(&self.0).into();
            }
        });

        quote! { #(#rules)* }
    };

    // quote
    let q = quote! {
        fn sanitize_auto(&mut self) {
            #inner
        }
    };

    Implementor::new(&node.def, t)
        .set_tokens(q)
        .to_token_stream()
}
