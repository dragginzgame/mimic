use super::Implementor;
use crate::node::{Enum, Newtype, Trait, TypeValidator};
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
    // validators
    let rules = validators(&node.validators);

    // inner
    let inner = if rules.is_empty() {
        quote!(Ok(()))
    } else {
        quote! {
            let mut errs = ::mimic::types::ErrorVec::new();
            #rules

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

// validators
// takes a slice of Validators and turns it into the #inner of the function
fn validators(validators: &[TypeValidator]) -> TokenStream {
    let rules: Vec<TokenStream> = validators
        .iter()
        .map(|val| {
            let path = &val.path;
            let args = &val.args;
            quote! {
                errs.add_result(#path::validate(&self.0, #(#args),*));
            }
        })
        .collect();

    // inner
    if rules.is_empty() {
        quote!()
    } else {
        quote! {
            #( #rules )*
        }
    }
}
