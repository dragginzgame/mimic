use super::Implementor;
use crate::node::{
    Cardinality, Entity, Enum, EnumVariant, FieldList, MacroNode, Map, Newtype, Record, Trait,
    Tuple, Value,
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Expr, Ident};

//
// Visitable
// the code that allows a Visitor to recurse down into a nested ORM type
//

///
/// NODE TYPES
///

// entity
pub fn entity(node: &Entity, t: Trait) -> TokenStream {
    let q = field_list(&node.fields);

    Implementor::new(node.def(), t)
        .set_tokens(q)
        .to_token_stream()
}

// record
pub fn record(node: &Record, t: Trait) -> TokenStream {
    let q = field_list(&node.fields);

    Implementor::new(node.def(), t)
        .set_tokens(q)
        .to_token_stream()
}

// enum
pub fn enum_(node: &Enum, t: Trait) -> TokenStream {
    // build inner
    let mut inner = quote!();
    for variant in &node.variants {
        inner.extend(enum_variant(variant));
    }
    let inner = quote! {
        match self {
            #inner
            _ => {},
        }
    };
    let q = drive_inner(&inner);

    Implementor::new(&node.def, t)
        .set_tokens(q)
        .to_token_stream()
}

// newtype
pub fn newtype(node: &Newtype, t: Trait) -> TokenStream {
    let var_expr: Expr = syn::parse_str("self.0").expect("can parse");
    let inner = quote_value(&node.value, &var_expr, "");
    let q = drive_inner(&inner);

    Implementor::new(&node.def, t)
        .set_tokens(q)
        .to_token_stream()
}

// map
pub fn map(node: &Map, t: Trait) -> TokenStream {
    let inner = quote! {
        for (map_key, map_value) in self.0.iter() {
            let key = map_key.to_string();
            ::mimic::orm::visit::perform_visit(visitor, map_key, &key);
            ::mimic::orm::visit::perform_visit(visitor, map_value, &key);
        }
    };
    let q = drive_inner(&inner);

    Implementor::new(&node.def, t)
        .set_tokens(q)
        .to_token_stream()
}

//
// FIELD TYPES
//
// Checks the cardinality of a value and prints out the corresponding
// visitor code
//

// field_list
pub fn field_list(node: &FieldList) -> TokenStream {
    let mut inner = quote!();
    for f in &node.fields {
        let var = format!("self.{}", f.name);
        let key = f.name.to_string();
        let var_expr: Expr = syn::parse_str(&var).expect("can parse");

        inner.extend(quote_value(&f.value, &var_expr, &key));
    }

    drive_inner(&inner)
}

// tuple
pub fn tuple(node: &Tuple, t: Trait) -> TokenStream {
    let mut inner = quote!();
    for (i, value) in node.values.iter().enumerate() {
        let var = format!("self.0.{i}");
        let key = format!("{i}");
        let var_expr: Expr = syn::parse_str(&var).expect("can parse");

        inner.extend(quote_value(value, &var_expr, &key));
    }
    let q = drive_inner(&inner);

    Implementor::new(&node.def, t)
        .set_tokens(q)
        .to_token_stream()
}

// quote_value
fn quote_value(value: &Value, var: &syn::Expr, name: &str) -> TokenStream {
    match value.cardinality() {
        Cardinality::One => quote! {
            ::mimic::orm::visit::perform_visit(visitor, &#var, #name);
        },
        Cardinality::Opt => quote! {
            if let Some(value) = #var.as_ref() {
                ::mimic::orm::visit::perform_visit(visitor, value, #name);
            }
        },
        Cardinality::Many => quote! {
            for value in #var.iter() {
                ::mimic::orm::visit::perform_visit(visitor, value, #name);
            }
        },
    }
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
            Self::#ident(value) => ::mimic::orm::visit::perform_visit(visitor, value, #name),
        },
        Cardinality::Opt => quote! {
            Self::#ident(option_value) => if let Some(value) = option_value {
                ::mimic::orm::visit::perform_visit(visitor, value, #name);
            },
        },
        Cardinality::Many => quote! {
            Self::#ident(values) => for value in values {
                ::mimic::orm::visit::perform_visit(visitor, value, #name);
            },
        },
    }
}

///
/// HELPERS
///

// drive_inner
// to eliminate a lot of repeating code shared between Node types
fn drive_inner(inner: &TokenStream) -> TokenStream {
    let visitor = if inner.is_empty() {
        quote!(_)
    } else {
        quote!(visitor)
    };

    quote! {
        fn drive(&self, #visitor: &mut dyn ::mimic::orm::visit::Visitor) {
            #inner
        }
    }
}
