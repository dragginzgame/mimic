use crate::prelude::*;
use quote::format_ident;

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

                let tokens = quote! {
                    #self_tokens
                    #child_tokens
                };

                let tokens = Implementor::new(node.def(), Trait::ValidateAuto)
                    .add_tokens(tokens)
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
                let ident = v.effective_ident();
                let ident_str = format!("{ident}");
                quote! {
                    Self::#ident => Err(format!("unspecified variant: {}", #ident_str).into()),
                }
            })
            .collect();

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
            #[doc = "Auto-generated validation for unspecified variants."]
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
        let list_rules = generate_validators_inner(&node.ty.validators, quote!(&self.0));
        let item_rules = generate_validators_inner(&node.item.validators, quote!(v)).map(|block| {
            let __item = format_ident!("__item");
            quote! {
                for #__item in &self.0 {
                    let v = #__item;
                    #block
                }
            }
        });

        match (list_rules, item_rules) {
            (None, None) => fn_wrap(None),
            (l, i) => {
                let l = l.unwrap_or_default();
                let i = i.unwrap_or_default();
                fn_wrap(Some(quote! { #l #i }))
            }
        }
    }
}

///
/// Map
///

impl ValidateAutoFn for Map {
    fn child_tokens(node: &Self) -> TokenStream {
        let key_rules = generate_validators_inner(&node.key.validators, quote!(k));
        let value_rules = generate_value_validation_inner(&node.value, quote!(v));

        match (key_rules, value_rules) {
            (None, None) => fn_wrap(None),
            (k, v) => {
                let k = k.unwrap_or_default();
                let v = v.unwrap_or_default();
                fn_wrap(Some(quote! {
                    for (k, v) in &self.0 {
                        #k
                        #v
                    }
                }))
            }
        }
    }
}

///
/// Newtype
///

impl ValidateAutoFn for Newtype {
    fn child_tokens(node: &Self) -> TokenStream {
        let type_rules = generate_validators_inner(&node.ty.validators, quote!(&self.0));
        let item_rules = generate_validators_inner(&node.item.validators, quote!(&self.0));

        match (type_rules, item_rules) {
            (None, None) => fn_wrap(None),
            (t, i) => {
                let t = t.unwrap_or_default();
                let i = i.unwrap_or_default();
                fn_wrap(Some(quote! { #t #i }))
            }
        }
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
        let set_rules = generate_validators_inner(&node.ty.validators, quote!(&self.0));
        let item_rules = generate_validators_inner(&node.item.validators, quote!(v)).map(|block| {
            let __item = format_ident!("__item");
            quote! {
                for #__item in &self.0 {
                    let v = #__item;
                    #block
                }
            }
        });

        match (set_rules, item_rules) {
            (None, None) => fn_wrap(None),
            (s, i) => {
                let s = s.unwrap_or_default();
                let i = i.unwrap_or_default();
                fn_wrap(Some(quote! { #s #i }))
            }
        }
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
        Some(quote! { #(#field_validations)* })
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

    let validator_exprs = generate_validators(validators, var_expr.clone());

    Some(quote! {
        let v = #var_expr;
        #(#validator_exprs;)*
    })
}

///
/// Shared cardinality wrapper
///
fn cardinality_wrapper(
    card: Cardinality,
    rules: Vec<TokenStream>,
    var_expr: TokenStream,
) -> Option<TokenStream> {
    if rules.is_empty() {
        return None;
    }

    let block = quote! { #(#rules;)* };

    let tokens = match card {
        Cardinality::One => quote! {
            let v = #var_expr;
            #block
        },
        Cardinality::Opt => quote! {
            if let Some(v) = #var_expr {
                #block
            }
        },
        Cardinality::Many => {
            let __item = format_ident!("__item");
            quote! {
                for #__item in #var_expr {
                    let v = #__item;
                    #block
                }
            }
        }
    };

    Some(tokens)
}

///
/// Generate full validation logic for a Value (Item + Cardinality)
///
fn generate_value_validation_inner(value: &Value, var_expr: TokenStream) -> Option<TokenStream> {
    let rules = generate_validators(&value.item.validators, quote!(v));
    cardinality_wrapper(value.cardinality(), rules, var_expr)
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
    cardinality_wrapper(value.cardinality(), rules, var_expr)
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
            #[doc = "Auto-generated recursive validation method."]
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
