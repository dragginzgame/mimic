use super::Implementor;
use crate::node::{Newtype, PrimitiveGroup, Trait};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

// newtype
pub fn newtype(node: &Newtype, t: Trait) -> TokenStream {
    // Generate inner logic
    let inner = if node.sanitizers.is_empty() {
        quote!(Ok(()))
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

            let constructor = if args.len() == 0 {
                quote! { #path::default() }
            } else {
                quote! { #path::new(#(#args),*) }
            };

            quote! {
                match #constructor.#sanitize_fn(&self.0) {
                    Ok(v) => self.0 = v.into(),
                    Err(e) => errs.add(e),
                }
            }
        });

        quote! {
            let mut errs = ::mimic::orm::types::ErrorVec::new();
            #( #rules )*
            errs.result()
        }
    };

    // quote
    let q = quote! {
        fn sanitize_auto(&mut self) -> ::std::result::Result<(), ::mimic::orm::types::ErrorVec> {
            #inner
        }
    };

    Implementor::new(&node.def, t)
        .set_tokens(q)
        .to_token_stream()
}
