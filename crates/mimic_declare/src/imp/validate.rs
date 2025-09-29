use crate::prelude::*;

///
/// ValidateAuto
///

pub struct ValidateAutoTrait;

pub trait ValidateAutoFn {
    fn self_tokens(_: &Self) -> TokenStream {
        quote!()
    }

    fn child_tokens(_: &Self) -> TokenStream {
        quote!()
    }
}

macro_rules! impl_validate_auto {
    ($ty:ty) => {
        impl Imp<$ty> for ValidateAutoTrait {
            fn strategy(node: &$ty) -> Option<TraitStrategy> {
                let self_tokens = ValidateAutoFn::self_tokens(node);
                let child_tokens = ValidateAutoFn::child_tokens(node);

                let tokens = Implementor::new(node.def(), Trait::ValidateAuto)
                    .add_tokens(self_tokens)
                    .add_tokens(child_tokens)
                    .to_token_stream();

                Some(TraitStrategy::from_impl(tokens))
            }
        }
    };
}

impl_validate_auto!(Entity);
impl_validate_auto!(Enum);
impl_validate_auto!(List);
impl_validate_auto!(Map);
impl_validate_auto!(Newtype);
impl_validate_auto!(Record);
impl_validate_auto!(Set);

///
/// Entity
///

impl ValidateAutoFn for Entity {
    fn child_tokens(node: &Self) -> TokenStream {
        fn_wrap(field_list(&node.fields))
    }
}

///
/// Enum
/// any variants that have the invalid flag set should not
/// pass validation if selected
///

impl ValidateAutoFn for Enum {
    fn self_tokens(node: &Self) -> TokenStream {
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

///
/// List
///

impl ValidateAutoFn for List {
    fn child_tokens(node: &Self) -> TokenStream {
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

impl ValidateAutoFn for Map {
    fn child_tokens(node: &Self) -> TokenStream {
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

impl ValidateAutoFn for Newtype {
    fn child_tokens(node: &Self) -> TokenStream {
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

impl ValidateAutoFn for Record {
    fn child_tokens(node: &Self) -> TokenStream {
        fn_wrap(field_list(&node.fields))
    }
}

///
/// Set
///

impl ValidateAutoFn for Set {
    fn child_tokens(node: &Self) -> TokenStream {
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
            let field_key = quote_one(&field.ident, to_str_lit);
            generate_field_value_validation_inner(
                &field.value,
                quote!(&self.#field_ident),
                &field_key,
            )
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

///
/// Generate validation logic for a specific record field so error entries carry the field key.
///
fn generate_field_value_validation_inner(
    value: &Value,
    var_expr: TokenStream,
    field_key: &TokenStream,
) -> Option<TokenStream> {
    let rules = generate_field_validators(&value.item.validators, quote!(v), field_key);

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

fn generate_field_validators(
    validators: &[TypeValidator],
    var_expr: TokenStream,
    field_key: &TokenStream,
) -> Vec<TokenStream> {
    validators
        .iter()
        .map(|validator| {
            let constructor = validator.quote_constructor();
            quote! {
                if let Err(err) = #constructor.validate(#var_expr) {
                    errs.add_for(#field_key, err);
                }
            }
        })
        .collect()
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
