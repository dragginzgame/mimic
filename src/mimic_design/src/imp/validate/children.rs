use crate::{
    imp::ImpFn,
    node::{Entity, FieldList, List, Map, Newtype, Record, Set, TypeValidator},
};
use proc_macro2::TokenStream;
use quote::quote;

///
/// ValidateChildrenFunction
///

pub struct ValidateChildrenFunction {}

///
/// Entity
///

impl ImpFn<Entity> for ValidateChildrenFunction {
    fn tokens(node: &Entity) -> TokenStream {
        field_list(&node.fields)
    }
}

///
/// List
///

impl ImpFn<List> for ValidateChildrenFunction {
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

        wrap_validate_fn(inner)
    }
}

///
/// Map
///

impl ImpFn<Map> for ValidateChildrenFunction {
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

        wrap_validate_fn(inner)
    }
}

///
/// Newtype
/// technically a newtype can have validation rules in two places
///

impl ImpFn<Newtype> for ValidateChildrenFunction {
    fn tokens(node: &Newtype) -> TokenStream {
        let type_rules = generate_validation_rules(&node.ty.validators, quote!(&self.0));
        let item_rules = generate_validation_rules(&node.item.validators, quote!(&self.0));
        let rules: Vec<TokenStream> = type_rules.into_iter().chain(item_rules).collect();

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

        wrap_validate_fn(inner)
    }
}

///
/// Record
///

impl ImpFn<Record> for ValidateChildrenFunction {
    fn tokens(node: &Record) -> TokenStream {
        field_list(&node.fields)
    }
}

///
/// Set
///

impl ImpFn<Set> for ValidateChildrenFunction {
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

        wrap_validate_fn(inner)
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

    wrap_validate_fn(inner)
}

// generate_validation_rules
// pass in a list of TypeValidators and then the variable to validate them by
fn generate_validation_rules(
    validators: &[TypeValidator],
    target_expr: TokenStream,
) -> Vec<TokenStream> {
    let mut rules = Vec::new();

    for val in validators {
        let constructor = val.quote_constructor();

        rules.push(quote! {
            errs.add_result(#constructor.validate(#target_expr));
        });
    }

    rules
}

// wrap_validate_fn
fn wrap_validate_fn(inner: TokenStream) -> TokenStream {
    quote! {
        fn validate_children(&self) -> ::std::result::Result<(), ::mimic::types::ErrorTree> {
            #inner
        }
    }
}
