use super::Implementor;
use crate::node::{Enum, Newtype, PrimitiveGroup, Trait};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

// enum_
// any variants that have the invalid flag set should not pass validation if selected
pub fn enum_(node: &Enum, t: Trait) -> TokenStream {
    let invalid_arms: TokenStream = node
        .variants
        .iter()
        .filter(|v| v.unspecified)
        .map(|v| {
            let name = format!("{}", v.name);
            let ident = format_ident!("{}", v.name);
            quote! {
                Self::#ident => Err(format!("unspecified variant: {}", #name).into()),
            }
        })
        .collect();

    // dont need a match if there's only one option
    let inner = if invalid_arms.is_empty() {
        quote!(Ok(()))
    } else {
        quote! {
            match &self {
                #invalid_arms
                _ => Ok(()),
            }
        }
    };
    let q = quote! {
        fn validate_auto(&self) -> ::std::result::Result<(), ::mimic::orm::types::ErrorVec> {
            #inner
        }
    };

    Implementor::new(&node.def, t)
        .set_tokens(q)
        .to_token_stream()
}

// newtype
pub fn newtype(node: &Newtype, t: Trait) -> TokenStream {
    // Generate inner logic
    let inner = if node.validators.is_empty() {
        quote!(Ok(()))
    } else {
        // validate function name
        let validate_fn = match node.primitive {
            Some(prim) => match prim.group() {
                PrimitiveGroup::Blob => quote! { validate_blob },
                PrimitiveGroup::Decimal | PrimitiveGroup::Integer => quote! { validate_number },
                PrimitiveGroup::String => quote! { validate_string },

                _ => panic!("validator error - invalid primitive group"),
            },
            None => panic!("validator error - no primitive"),
        };

        // Generate rules
        let rules = node.validators.iter().map(|val| {
            let path = &val.path;
            let args = &val.args;

            let constructor = if args.len() == 0 {
                quote! { #path::default() }
            } else {
                quote! { #path::new(#(#args),*) }
            };

            quote! {
                errs.add_result(#constructor.#validate_fn(&self.0));
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
        fn validate_auto(&self) -> ::std::result::Result<(), ::mimic::orm::types::ErrorVec> {
            #inner
        }
    };

    Implementor::new(&node.def, t)
        .set_tokens(q)
        .to_token_stream()
}
