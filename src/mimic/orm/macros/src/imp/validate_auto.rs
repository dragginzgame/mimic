use crate::{
    imp::{Implementor, Trait},
    node::{Enum, Newtype, TypeValidator},
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

// enum_
// any variants that have the invalid flag set should not pass validation if selected
pub fn enum_(node: &Enum, t: Trait) -> TokenStream {
    let invalid_arms: TokenStream = node
        .variants
        .iter()
        .filter(|v| v.invalid)
        .map(|v| {
            let name = format!("{}", v.name);
            let ident = format_ident!("{}", v.name);
            quote! {
                Self::#ident => Err(format!("invalid variant: {}", #name).into()),
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

// newtype
// validators + guide logic
pub fn newtype(node: &Newtype, t: Trait) -> TokenStream {
    // validators
    let mut rules = validators(&node.validators);

    // guide
    if let Some(guide) = &node.guide {
        let values: Vec<_> = guide
            .entries
            .iter()
            .map(|entry| {
                let ev = &entry.value;
                quote! { #ev }
            })
            .collect();

        rules.extend(quote! {
            let valid_values = [#(#values),*];

            match NumCast::from(self.0) {
                Some(value) => {
                    if !valid_values.contains(&value) {
                        errs.add(format!("value {} does not appear in guide", &self.0));
                    }
                }
                None => errs.add("failed to convert value to isize")
            }
        });
    };

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
