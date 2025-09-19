use crate::{
    imp::{Imp, Implementor, Trait, TraitStrategy},
    node::{Entity, Enum, EnumVariant, FieldList, Newtype, Record, Tuple},
    traits::HasDef,
};
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::{Index, LitStr};

///
/// VisitableTrait
///

pub struct VisitableTrait {}

///
/// Entity
///

impl Imp<Entity> for VisitableTrait {
    fn strategy(node: &Entity) -> Option<TraitStrategy> {
        let q = field_list(&node.fields);

        let tokens = Implementor::new(node.def(), Trait::Visitable)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// Enum
///

impl Imp<Enum> for VisitableTrait {
    fn strategy(node: &Enum) -> Option<TraitStrategy> {
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

        let tokens = Implementor::new(node.def(), Trait::Visitable)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// Newtype
///

impl Imp<Newtype> for VisitableTrait {
    fn strategy(node: &Newtype) -> Option<TraitStrategy> {
        let inner = quote! {
           perform_visit(visitor, &self.0, None);
        };

        let q = quote_drive_method(&inner);

        let tokens = Implementor::new(node.def(), Trait::Visitable)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// Record
///

impl Imp<Record> for VisitableTrait {
    fn strategy(node: &Record) -> Option<TraitStrategy> {
        let q = field_list(&node.fields);

        let tokens = Implementor::new(node.def(), Trait::Visitable)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// Tuple
///

impl Imp<Tuple> for VisitableTrait {
    fn strategy(node: &Tuple) -> Option<TraitStrategy> {
        let mut inner = quote!();

        for (i, _) in node.values.iter().enumerate() {
            let index = Index::from(i);
            let key_lit = LitStr::new(&i.to_string(), Span::call_site());

            inner.extend(quote! {
                perform_visit(visitor, &self.#index, #key_lit);
            });
        }

        let q = quote_drive_method(&inner);

        let tokens = Implementor::new(node.def(), Trait::Visitable)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
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
        let field_ident = format_ident!("{}", f.ident);
        let field_ident_s = field_ident.to_string();

        inner.extend(quote! {
            perform_visit(visitor, &self.#field_ident, #field_ident_s);
        });
    }

    quote_drive_method(&inner)
}

// enum_variant
pub fn enum_variant(variant: &EnumVariant) -> TokenStream {
    let name = &variant.name;

    if variant.value.is_some() {
        let name_string = name.to_string();

        quote! {
            Self::#name(value) => perform_visit(visitor, value, #name_string),
        }
    } else {
        quote!(Self::#name => {})
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
            use ::mimic::core::visit::perform_visit;

            #inner
        }
    }
}
