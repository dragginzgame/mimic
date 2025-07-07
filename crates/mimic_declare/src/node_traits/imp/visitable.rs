use crate::{
    node::{Entity, Enum, EnumVariant, FieldList, List, Map, Newtype, Record, Set, Tuple},
    node_traits::{Imp, Implementor, Trait},
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Expr, parse_str};

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
            }
        };

        let q = quote_drive_method(&inner);

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
        let inner = quote! {
            for (i, value) in self.0.iter().enumerate() {
                let visitor_key = i.to_string();

                ::mimic::core::visit::perform_visit(visitor, value, &visitor_key);
            }
        };

        let q = quote_drive_method(&inner);

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
                let key_path = format!("{}:key", visitor_key);
                let val_path = format!("{}:val", visitor_key);

                ::mimic::core::visit::perform_visit(visitor, k, &key_path);
                ::mimic::core::visit::perform_visit(visitor, v, &val_path);
            }
        };
        let q = quote_drive_method(&inner);

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
        let inner = quote! {
            ::mimic::core::visit::perform_visit(visitor, &self.0, "");
        };

        let q = quote_drive_method(&inner);

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
/// Set
///

impl Imp<Set> for VisitableTrait {
    fn tokens(node: &Set, t: Trait) -> Option<TokenStream> {
        let inner = quote! {
            for (i, item) in self.0.iter().enumerate() {
                let visitor_key = i.to_string();
                ::mimic::core::visit::perform_visit(visitor, item, &visitor_key);
            }
        };
        let q = quote_drive_method(&inner);

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

        for (i, _) in node.values.iter().enumerate() {
            let key = i.to_string();
            let var: syn::Expr =
                syn::parse_str(&format!("self.{i}")).expect("can parse tuple field");

            inner.extend(quote! {
                ::mimic::core::visit::perform_visit(visitor, &#var, #key);
            });
        }

        let q = quote_drive_method(&inner);

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}
///
/// SUB TYPES
///
/// Checks the cardinality of a value and prints out the corresponding
/// visitor code
///

// field_list
pub fn field_list(fields: &FieldList) -> TokenStream {
    let mut inner = quote!();

    for f in fields {
        let field_name = f.name.to_string();
        let var_expr: Expr =
            parse_str(&format!("self.{field_name}")).expect("can parse field access");

        inner.extend(quote! {
            ::mimic::core::visit::perform_visit(visitor, &#var_expr, #field_name);
        });
    }

    quote_drive_method(&inner)
}

// enum_variant
pub fn enum_variant(variant: &EnumVariant) -> TokenStream {
    let name = &variant.name;

    match &variant.value {
        Some(_) => {
            let name_string = name.to_string();

            quote! {
                Self::#name(value) => ::mimic::core::visit::perform_visit(visitor, value, #name_string),
            }
        }
        None => quote!(Self::#name => {}),
    }
}

///
/// HELPERS
///

// quote_drive_method
// to eliminate a lot of repeating code shared between Node types
fn quote_drive_method(inner: &TokenStream) -> TokenStream {
    quote! {
        fn drive(&self, visitor: &mut dyn ::mimic::core::visit::Visitor) {
            #inner
        }
    }
}
