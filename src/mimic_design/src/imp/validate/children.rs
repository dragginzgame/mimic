use crate::{
    imp::ImpFn,
    node::{Cardinality, Entity, FieldList, List, Map, Newtype, Record, Set, TypeValidator, Value},
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
        wrap_validate_fn(field_list(&node.fields))
    }
}

///
/// List
///

impl ImpFn<List> for ValidateChildrenFunction {
    fn tokens(node: &List) -> TokenStream {
        generate_validators_inner(&node.item.validators, quote!(v))
            .map(|inner| {
                wrap_validate_fn(quote! {
                    for v in &self.0 {
                        #inner
                    }
                })
            })
            .unwrap_or_else(wrap_validate_stub)
    }
}

///
/// Map
///

impl ImpFn<Map> for ValidateChildrenFunction {
    fn tokens(node: &Map) -> TokenStream {
        let key_rules = generate_validators_inner(&node.key.validators, quote!(k));
        let value_rules = generate_value_validation_inner(&node.value, quote!(v));

        if key_rules.is_none() && value_rules.is_none() {
            return wrap_validate_stub();
        }

        let inner = quote! {
            for (k, v) in &self.0 {
                #key_rules
                #value_rules
            }
        };

        wrap_validate_fn(inner)
    }
}

///
/// Newtype
///

impl ImpFn<Newtype> for ValidateChildrenFunction {
    fn tokens(node: &Newtype) -> TokenStream {
        let type_rules = generate_validators_inner(&node.ty.validators, quote!(&self.0));
        let item_rules = generate_validators_inner(&node.item.validators, quote!(&self.0));

        if type_rules.is_none() && item_rules.is_none() {
            return wrap_validate_stub();
        }

        wrap_validate_fn(quote! {
            #type_rules
            #item_rules
        })
    }
}

///
/// Record
///

impl ImpFn<Record> for ValidateChildrenFunction {
    fn tokens(node: &Record) -> TokenStream {
        wrap_validate_fn(field_list(&node.fields))
    }
}

///
/// Set
///

impl ImpFn<Set> for ValidateChildrenFunction {
    fn tokens(node: &Set) -> TokenStream {
        generate_validators_inner(&node.item.validators, quote!(v))
            .map(|inner| {
                wrap_validate_fn(quote! {
                    for v in &self.0 {
                        #inner
                    }
                })
            })
            .unwrap_or_else(wrap_validate_stub)
    }
}

///
/// Helper Functions
///

fn field_list(fields: &FieldList) -> TokenStream {
    let field_validations: Vec<TokenStream> = fields
        .fields
        .iter()
        .filter_map(|field| {
            let field_ident = &field.name;
            generate_value_validation_inner(&field.value, quote!(&self.#field_ident))
        })
        .collect();

    if field_validations.is_empty() {
        return quote!();
    }

    quote! {
        #(#field_validations)*
    }
}

///
/// Generate validation rules from a list of validators
///
fn generate_validators(validators: &[TypeValidator], var_expr: TokenStream) -> Vec<TokenStream> {
    validators
        .iter()
        .map(|validator| {
            let constructor = validator.quote_constructor();
            quote! {
                errs.add_result(#constructor.validate(#var_expr));
            }
        })
        .collect()
}

///
/// Generate full validation logic from a list of validators
///
fn generate_validators_inner(
    validators: &[TypeValidator],
    var_expr: TokenStream,
) -> Option<TokenStream> {
    let rules = generate_validators(validators, quote!(v));

    if rules.is_empty() {
        None
    } else {
        Some(quote! {
            let mut errs = ::mimic::types::ErrorTree::new();
            let v = #var_expr;
            #(#rules)*
            errs.result()?;
        })
    }
}

///
/// Generate full validation logic for a Value (Item + Cardinality)
///
fn generate_value_validation_inner(value: &Value, var_expr: TokenStream) -> Option<TokenStream> {
    let rules = generate_validators(&value.item.validators, quote!(v));

    if rules.is_empty() {
        return None;
    }

    let body = match value.cardinality() {
        Cardinality::One => quote! {
            let v = #var_expr;
            #(#rules)*
        },
        Cardinality::Opt => quote! {
            if let Some(v) = #var_expr {
                #(#rules)*
            }
        },
        Cardinality::Many => quote! {
            for v in #var_expr {
                #(#rules)*
            }
        },
    };

    Some(quote! {
        let mut errs = ::mimic::types::ErrorTree::new();
        #body

        errs.result()?;
    })
}

///
/// Emit validate_children with body
///
fn wrap_validate_fn(inner: TokenStream) -> TokenStream {
    quote! {
        fn validate_children(&self) -> ::std::result::Result<(), ::mimic::types::ErrorTree> {
            #inner

            Ok(())
        }
    }
}

///
/// Emit a no-op validate_children if no validators exist
///
fn wrap_validate_stub() -> TokenStream {
    quote! {
        fn validate_children(&self) -> ::std::result::Result<(), ::mimic::types::ErrorTree> {
            Ok(())
        }
    }
}
