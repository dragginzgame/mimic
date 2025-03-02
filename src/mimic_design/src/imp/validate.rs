use crate::{
    imp::{Imp, Implementor},
    node::{Entity, Enum, FieldList, Newtype, Record, Trait},
};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};

///
/// ValidateAutoTrait
///

pub struct ValidateAutoTrait {}

///
/// Entity
///

impl Imp<Entity> for ValidateAutoTrait {
    fn tokens(node: &Entity, t: Trait) -> Option<TokenStream> {
        let q = field_list(&node.fields);

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// Record
///

impl Imp<Record> for ValidateAutoTrait {
    fn tokens(node: &Record, t: Trait) -> Option<TokenStream> {
        let q = field_list(&node.fields);

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

// field_list
// check if a node's fields are empty and generate an appropriate logical expression
pub fn field_list(node: &FieldList) -> TokenStream {
    // Generate rules
    let rules: Vec<_> = node
        .fields
        .iter()
        .flat_map(|field| {
            field.validators.iter().map(move |val| {
                let field_ident = &field.name;
                let path = &val.path;
                let args = &val.args;

                let validator = if args.is_empty() {
                    quote! { #path::default() }
                } else {
                    quote! { #path::new(#(#args),*) }
                };

                // pass self.field to the validator
                quote! {
                    if let Err(e) = #validator.validate(&self.#field_ident) {
                        errs.add(format!("field {} {e}", stringify!(#field_ident)));
                    }
                }
            })
        })
        .collect();

    // inner
    let inner = if rules.is_empty() {
        quote!(Ok(()))
    } else {
        quote! {
            let mut errs = ::mimic::types::ErrorTree::new();
            #( #rules )*

            errs.result()
        }
    };

    quote! {
        fn validate_auto(&self) -> ::std::result::Result<(), ::mimic::types::ErrorTree> {
            #inner
        }
    }
}

///
/// Enum
/// any variants that have the invalid flag set should not pass validation if selected
///

impl Imp<Enum> for ValidateAutoTrait {
    fn tokens(node: &Enum, t: Trait) -> Option<TokenStream> {
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
            fn validate_auto(&self) -> ::std::result::Result<(), ::mimic::types::ErrorTree> {
                #inner
            }
        };

        let imp = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(imp)
    }
}

///
/// Newtype
///

impl Imp<Newtype> for ValidateAutoTrait {
    fn tokens(node: &Newtype, t: Trait) -> Option<TokenStream> {
        // Generate rules
        let rules: Vec<_> = node
            .ty
            .validators
            .iter()
            .map(|val| {
                let path = &val.path;
                let args = &val.args;

                let validator = if args.is_empty() {
                    quote! { #path::default() }
                } else {
                    quote! { #path::new(#(#args),*) }
                };

                quote! {
                    errs.add_result(#validator.validate(&self.0));
                }
            })
            .collect();

        // inner
        let inner = if rules.is_empty() {
            quote!(Ok(()))
        } else {
            quote! {
                let mut errs = ::mimic::types::ErrorTree::new();
                #( #rules )*

                errs.result()
            }
        };

        // quote
        let q = quote! {
            fn validate_auto(&self) -> ::std::result::Result<(), ::mimic::types::ErrorTree> {
                #inner
            }
        };

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}
