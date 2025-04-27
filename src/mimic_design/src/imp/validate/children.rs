use crate::{
    imp::ImpFn,
    node::{Entity, FieldList, List, Map, Newtype, Record, Set, TypeValidator, Value},
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
        let inner = generate_validators_inner(&node.item.validators, quote!(v));

        wrap_validate_fn(quote! {
            for v in &self.0 {
                #inner
            }
        })
    }
}

///
/// Map
///

impl ImpFn<Map> for ValidateChildrenFunction {
    fn tokens(node: &Map) -> TokenStream {
        let key_rules = generate_validators_inner(&node.key.validators, quote!(k));
        let value_rules = generate_value_validation_inner(&node.value, quote!(v));

        wrap_validate_fn(quote! {
            for (k, v) in &self.0 {
                #key_rules
                #value_rules
            }
        })
    }
}

///
/// Newtype
///

impl ImpFn<Newtype> for ValidateChildrenFunction {
    fn tokens(node: &Newtype) -> TokenStream {
        let type_rules = generate_validators_inner(&node.ty.validators, quote!(&self.0));
        let item_rules = generate_validators_inner(&node.item.validators, quote!(&self.0));

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
        field_list(&node.fields)
    }
}

///
/// Set
///

impl ImpFn<Set> for ValidateChildrenFunction {
    fn tokens(node: &Set) -> TokenStream {
        let inner = generate_validators_inner(&node.item.validators, quote!(v));

        wrap_validate_fn(quote! {
            for v in &self.0 {
                #inner
            }
        })
    }
}

///
/// Helper Functions
///

fn field_list(fields: &FieldList) -> TokenStream {
    let field_validations: Vec<TokenStream> = fields
        .fields
        .iter()
        .map(|field| {
            let field_ident = &field.name;
            let validation =
                generate_value_validation_inner(&field.value, quote!(&self.#field_ident));
            quote! {
                {
                    #validation
                }
            }
        })
        .collect();

    wrap_validate_fn(quote! {
        #(#field_validations)*
    })
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
fn generate_validators_inner(validators: &[TypeValidator], var_expr: TokenStream) -> TokenStream {
    let rules = generate_validators(validators, quote!(v));

    if rules.is_empty() {
        quote! {}
    } else {
        quote! {
            let mut errs = ::mimic::types::ErrorTree::new();
            let v = #var_expr;
            #(#rules)*
            errs.result()?;
        }
    }
}

///
/// Generate full validation logic for a Value (Item + Cardinality)
///
fn generate_value_validation_inner(value: &Value, var_expr: TokenStream) -> TokenStream {
    let rules = generate_validators(&value.item.validators, quote!(v));

    if rules.is_empty() {
        return quote! {};
    }

    match value.cardinality() {
        crate::node::Cardinality::One => {
            quote! {
                let mut errs = ::mimic::types::ErrorTree::new();
                {
                    let v = #var_expr;
                    #(#rules)*
                }
                errs.result()?;
            }
        }
        crate::node::Cardinality::Opt => {
            quote! {
                let mut errs = ::mimic::types::ErrorTree::new();
                if let Some(v) = #var_expr {
                    #(#rules)*
                }
                errs.result()?;
            }
        }
        crate::node::Cardinality::Many => {
            quote! {
                let mut errs = ::mimic::types::ErrorTree::new();
                for v in #var_expr {
                    #(#rules)*
                }
                errs.result()?;
            }
        }
    }
}

///
/// Wrap validate_children function
///
fn wrap_validate_fn(inner: TokenStream) -> TokenStream {
    quote! {
        fn validate_children(&self) -> ::std::result::Result<(), ::mimic::types::ErrorTree> {
            #inner

            Ok(())
        }
    }
}
