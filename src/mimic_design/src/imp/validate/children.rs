use crate::{
    imp::ImpFn,
    node::{Entity, FieldList, List, Map, Newtype, Record, Set, TypeValidator},
};
use proc_macro2::TokenStream;
use quote::quote;

///
/// ValidateChildFunction
///

pub struct ValidateChildFunction {}

///
/// Entity
///

impl ImpFn<Entity> for ValidateChildFunction {
    fn tokens(node: &Entity) -> TokenStream {
        field_list(&node.fields)
    }
}

///
/// List
///

impl ImpFn<List> for ValidateChildFunction {
    fn tokens(node: &List) -> TokenStream {
        let rules = generate_validation_rules(&node.item.validators, quote!(v));

        // inner
        let inner = if rules.is_empty() {
            quote!(Ok(()))
        } else {
            quote! {
                let mut errs = ::mimic::types::ErrorTree::new();
                for v in &self.0 {
                    #(#rules)*
                }

                errs.result()
            }
        };

        format_fn(inner)
    }
}

///
/// Map
///

impl ImpFn<Map> for ValidateChildFunction {
    fn tokens(node: &Map) -> TokenStream {
        // rules
        let mut rules = Vec::<TokenStream>::new();
        rules.extend(generate_validation_rules(&node.key.validators, quote!(k)));
        rules.extend(generate_validation_rules(
            &node.value.item.validators,
            quote!(v),
        ));

        // inner
        let inner = if rules.is_empty() {
            quote!(Ok(()))
        } else {
            quote! {
                let mut errs = ::mimic::types::ErrorTree::new();
                for (k, v) in &self.0 {
                    #(#rules)*
                }

                errs.result()
            }
        };

        format_fn(inner)
    }
}

///
/// Newtype
///

impl ImpFn<Newtype> for ValidateChildFunction {
    fn tokens(node: &Newtype) -> TokenStream {
        let rules = generate_validation_rules(&node.item.validators, quote!(&self.0));

        // inner
        let inner = if rules.is_empty() {
            quote!(Ok(()))
        } else {
            quote! {
                let mut errs = ::mimic::types::ErrorTree::new();
                #(#rules)*

                errs.result()
            }
        };

        format_fn(inner)
    }
}

///
/// Record
///

impl ImpFn<Record> for ValidateChildFunction {
    fn tokens(node: &Record) -> TokenStream {
        field_list(&node.fields)
    }
}

///
/// Set
///

impl ImpFn<Set> for ValidateChildFunction {
    fn tokens(node: &Set) -> TokenStream {
        let rules = generate_validation_rules(&node.item.validators, quote!(v));

        // inner
        let inner = if rules.is_empty() {
            quote!(Ok(()))
        } else {
            quote! {
                let mut errs = ::mimic::types::ErrorTree::new();
                for v in &self.0 {
                    #(#rules)*
                }

                errs.result()
            }
        };

        format_fn(inner)
    }
}

///
/// Helper Functions
///

// field_list
// check if a node's fields are empty and generate an appropriate logical expression
fn field_list(node: &FieldList) -> TokenStream {
    // Generate rules
    let rules: Vec<_> = node
        .fields
        .iter()
        .flat_map(|field| {
            field.value.item.validators.iter().map(move |val| {
                let field_ident = &field.name;
                let constructor = val.quote_constructor();

                // pass self.field to the validator
                quote! {
                    if let Err(e) = #constructor.validate(&self.#field_ident) {
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

    format_fn(inner)
}

// generate_validation_rules
// pass in a list of TypeValidators and then the variable to validate them by
fn generate_validation_rules(
    validators: &[TypeValidator],
    target_expr: TokenStream,
) -> Vec<TokenStream> {
    let mut rules = Vec::new();

    for val in validators.iter() {
        let constructor = val.quote_constructor();

        rules.push(quote! {
            errs.add_result(#constructor.validate(#target_expr));
        });
    }

    rules
}

// format_fn
fn format_fn(inner: TokenStream) -> TokenStream {
    quote! {
        fn validate_children(&self) -> ::std::result::Result<(), ::mimic::types::ErrorTree> {
            #inner
        }
    }
}
