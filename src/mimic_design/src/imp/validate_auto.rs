use super::Implementor;
use crate::node::{Enum, Newtype, PrimitiveGroup, Trait};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};

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
        fn validate_auto(&self) -> ::std::result::Result<(), ::mimic::types::ErrorVec> {
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
    let inner = if node.ty.validators.is_empty() {
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
        let rules = node.ty.validators.iter().map(|val| {
            let path = &val.path;
            let args = &val.args;

            let constructor = if args.is_empty() {
                quote! { #path::default() }
            } else {
                quote! { #path::new(#(#args),*) }
            };

            quote! {
                errs.add_result(#constructor.#validate_fn(&self.0));
            }
        });

        quote! {
            let mut errs = ::mimic::types::ErrorVec::new();
            #( #rules )*
            errs.result()
        }
    };

    // quote
    let q = quote! {
        fn validate_auto(&self) -> ::std::result::Result<(), ::mimic::types::ErrorVec> {
            #inner
        }
    };

    Implementor::new(&node.def, t)
        .set_tokens(q)
        .to_token_stream()
}
