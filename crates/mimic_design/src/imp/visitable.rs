use crate::{
    imp::{Imp, Implementor},
    node::{
        Entity, Enum, EnumVariant, Field, List, Map, Newtype, Record, Set, Trait, Tuple, Value,
    },
};
use mimic::schema::types::Cardinality;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Expr, Ident};

///
/// VisitableTrait
///

pub struct VisitableTrait {}

///
/// Entity
///

impl Imp<Entity> for VisitableTrait {
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

impl Imp<Record> for VisitableTrait {
    fn tokens(node: &Record, t: Trait) -> Option<TokenStream> {
        let q = field_list(&node.fields);

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// Enum
///

impl Imp<Enum> for VisitableTrait {
    fn tokens(node: &Enum, t: Trait) -> Option<TokenStream> {
        // build inner
        let mut variant_tokens = quote!();
        for variant in &node.variants {
            variant_tokens.extend(enum_variant(variant));
        }

        let inner = quote! {
            match self {
                #variant_tokens
                _ => {},
            }
        };

        let q = drive_inner(&inner);

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// List
///

impl Imp<List> for VisitableTrait {
    fn tokens(node: &List, t: Trait) -> Option<TokenStream> {
        let inner = quote_value(&self0_expr(), Cardinality::Many, "");
        let q = drive_inner(&inner);

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// Map
///

impl Imp<Map> for VisitableTrait {
    fn tokens(node: &Map, t: Trait) -> Option<TokenStream> {
        let inner = quote! {
            for (k, v) in self.0.iter() {
                let visitor_key = k.to_string();
                ::mimic::ops::visit::perform_visit(visitor, k, &visitor_key);
                ::mimic::ops::visit::perform_visit(visitor, v, &visitor_key);
            }
        };
        let q = drive_inner(&inner);

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// Newtype
///

impl Imp<Newtype> for VisitableTrait {
    fn tokens(node: &Newtype, t: Trait) -> Option<TokenStream> {
        let inner = quote_value(&self0_expr(), Cardinality::One, "");
        let q = drive_inner(&inner);

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// Set
///

impl Imp<Set> for VisitableTrait {
    fn tokens(node: &Set, t: Trait) -> Option<TokenStream> {
        let inner = quote! {
            for (i, item) in self.0.iter().enumerate() {
                let visitor_key = i.to_string();
                ::mimic::ops::visit::perform_visit(visitor, item, &visitor_key);
            }
        };
        let q = drive_inner(&inner);

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// Tuple
///

impl Imp<Tuple> for VisitableTrait {
    fn tokens(node: &Tuple, t: Trait) -> Option<TokenStream> {
        let mut inner = quote!();
        for (i, value) in node.values.iter().enumerate() {
            let var = format!("self.0.{i}");
            let key = format!("{i}");
            let var_expr: Expr = syn::parse_str(&var).expect("can parse");

            inner.extend(quote_value(&var_expr, value.cardinality(), &key));
        }
        let q = drive_inner(&inner);

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

//
// FIELD TYPES
//
// Checks the cardinality of a value and prints out the corresponding
// visitor code
//

// field_list
pub fn field_list(fields: &[Field]) -> TokenStream {
    let mut inner = quote!();
    for f in fields {
        let var = format!("self.{}", f.name);
        let key = f.name.to_string();
        let var_expr: Expr = syn::parse_str(&var).expect("can parse");

        inner.extend(quote_value(&var_expr, f.value.cardinality(), &key));
    }

    drive_inner(&inner)
}

///
/// VARIANT TYPES
///

// enum_variant
pub fn enum_variant(variant: &EnumVariant) -> TokenStream {
    let name = &variant.name;

    match &variant.value {
        Some(value) => {
            let inner = quote_variant(value, name);

            quote!(#inner)
        }
        None => quote!(),
    }
}

// quote_variant
fn quote_variant(value: &Value, ident: &Ident) -> TokenStream {
    let name = ident.to_string();
    match value.cardinality() {
        Cardinality::One => quote! {
            Self::#ident(value) => ::mimic::ops::visit::perform_visit(visitor, value, #name),
        },
        Cardinality::Opt => quote! {
            Self::#ident(option_value) => if let Some(value) = option_value {
                ::mimic::ops::visit::perform_visit(visitor, value, #name);
            },
        },
        Cardinality::Many => quote! {
            Self::#ident(values) => for value in values.iter() {
                ::mimic::ops::visit::perform_visit(visitor, value, #name);
            },
        },
    }
}

///
/// HELPERS
///

// self0_expr
fn self0_expr() -> Expr {
    syn::parse_str("self.0").expect("Failed to parse 'self.0'")
}

// drive_inner
// to eliminate a lot of repeating code shared between Node types
fn drive_inner(inner: &TokenStream) -> TokenStream {
    let visitor = if inner.is_empty() {
        quote!(_)
    } else {
        quote!(visitor)
    };

    quote! {
        fn drive(&self, #visitor: &mut dyn ::mimic::ops::visit::Visitor) {
            #inner
        }
    }
}

// quote_value
fn quote_value(var: &syn::Expr, cardinality: Cardinality, name: &str) -> TokenStream {
    match cardinality {
        Cardinality::One => quote! {
            ::mimic::ops::visit::perform_visit(visitor, &#var, #name);
        },
        Cardinality::Opt => quote! {
            if let Some(value) = #var.as_ref() {
                ::mimic::ops::visit::perform_visit(visitor, value, #name);
            }
        },
        Cardinality::Many => quote! {
            for value in #var.iter() {
                ::mimic::ops::visit::perform_visit(visitor, value, #name);
            }
        },
    }
}
