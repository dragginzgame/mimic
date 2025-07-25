use crate::{
    node::{Entity, FieldList, List, Map, Newtype, Record, Set, TypeValidator, Value},
    node_traits::ImpFn,
};
use mimic_schema::types::Cardinality;
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
        fn_wrap(field_list(&node.fields))
    }
}

///
/// List
///

impl ImpFn<List> for ValidateChildrenFunction {
    fn tokens(node: &List) -> TokenStream {
        let inner = generate_validators_inner(&node.item.validators, quote!(v)).map(|block| {
            quote! {
                for v in &self.0 {
                    #block
                }
            }
        });

        fn_wrap(inner)
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
            return fn_wrap(None);
        }

        let key_tokens = key_rules.unwrap_or_default();
        let val_tokens = value_rules.unwrap_or_default();

        fn_wrap(Some(quote! {
            for (k, v) in &self.0 {
                #key_tokens
                #val_tokens
            }
        }))
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
            return fn_wrap(None);
        }

        let type_tokens = type_rules.unwrap_or_default();
        let item_tokens = item_rules.unwrap_or_default();

        fn_wrap(Some(quote! {
            #type_tokens
            #item_tokens
        }))
    }
}

///
/// Record
///

impl ImpFn<Record> for ValidateChildrenFunction {
    fn tokens(node: &Record) -> TokenStream {
        fn_wrap(field_list(&node.fields))
    }
}

///
/// Set
///

impl ImpFn<Set> for ValidateChildrenFunction {
    fn tokens(node: &Set) -> TokenStream {
        let inner = generate_validators_inner(&node.item.validators, quote!(v)).map(|block| {
            quote! {
                for v in &self.0 {
                    #block
                }
            }
        });

        fn_wrap(inner)
    }
}

///
/// Helper Functions
///

fn field_list(fields: &FieldList) -> Option<TokenStream> {
    let field_validations: Vec<TokenStream> = fields
        .iter()
        .filter_map(|field| {
            let field_ident = &field.ident;
            generate_value_validation_inner(&field.value, quote!(&self.#field_ident))
        })
        .collect();

    if field_validations.is_empty() {
        None
    } else {
        Some(quote! {
            #(#field_validations)*
        })
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
    if validators.is_empty() {
        return None;
    }

    let validator_exprs = generate_validators(validators, quote!(v));

    Some(quote! {
        let v = #var_expr;
        #(#validator_exprs)*
    })
}

///
/// Generate full validation logic for a Value (Item + Cardinality)
///
fn generate_value_validation_inner(value: &Value, var_expr: TokenStream) -> Option<TokenStream> {
    let rules = generate_validators(&value.item.validators, quote!(v));

    if rules.is_empty() {
        return None;
    }

    let tokens = match value.cardinality() {
        Cardinality::One => {
            quote! {
                {
                    let v = #var_expr;
                    #(#rules)*
                }
            }
        }
        Cardinality::Opt => {
            quote! {
                if let Some(v) = #var_expr {
                    #(#rules)*
                }
            }
        }
        Cardinality::Many => {
            quote! {
                for v in #var_expr {
                    #(#rules)*
                }
            }
        }
    };

    Some(tokens)
}

// fn_wrap
fn fn_wrap(inner: Option<TokenStream>) -> TokenStream {
    if let Some(inner) = inner {
        quote! {
            fn validate_children(&self) -> ::std::result::Result<(), ::mimic::common::error::ErrorTree> {
                let mut errs = ::mimic::common::error::ErrorTree::new();
                #inner

                errs.result()
            }
        }
    } else {
        quote!()
    }
}
