use crate::{imp::ImpFn, node::Enum};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

///
/// ValidateSelfFn
///

pub struct ValidateSelfFn {}

///
/// Enum
///
/// any variants that have the invalid flag set should not
/// pass validation if selected
///

impl ImpFn<Enum> for ValidateSelfFn {
    fn tokens(node: &Enum) -> TokenStream {
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

        // inner
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

        quote! {
            fn validate_self(&self) -> ::std::result::Result<(), ::mimic::common::error::ErrorTree> {
                #inner
            }
        }
    }
}
