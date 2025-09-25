use crate::prelude::*;

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
        // Collect both immutable and mutable match arms
        let (arms, arms_mut): (TokenStream, TokenStream) =
            node.variants.iter().map(enum_variant).unzip();

        let inner = quote! { match self { #arms } };
        let inner_mut = quote! { match self { #arms_mut } };

        let tokens = Implementor::new(node.def(), Trait::Visitable)
            .set_tokens(quote_drives(&inner, &inner_mut))
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// List
///

impl Imp<List> for VisitableTrait {
    fn strategy(node: &List) -> Option<TraitStrategy> {
        let inner = quote! {
            for (i, v) in self.0.iter().enumerate() {
                perform_visit(visitor, v, i);
            }
        };

        let inner_mut = quote! {
            for (i, v) in self.0.iter_mut().enumerate() {
                perform_visit_mut(visitor, v, i);
            }
        };

        let q = quote_drives(&inner, &inner_mut);

        let tokens = Implementor::new(node.def(), Trait::Visitable)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// Map
///

impl Imp<Map> for VisitableTrait {
    fn strategy(node: &Map) -> Option<TraitStrategy> {
        let inner = quote! {
            for (i, (_k, v)) in self.0.iter().enumerate() {
                perform_visit(visitor, v, i);
            }
        };

        let inner_mut = quote! {
            for (i, (_k, v)) in self.0.iter_mut().enumerate() {
                perform_visit_mut(visitor, v, i);
            }
        };

        let q = quote_drives(&inner, &inner_mut);

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
        let inner_mut = quote! {
           perform_visit_mut(visitor, &mut self.0, None);
        };

        let q = quote_drives(&inner, &inner_mut);

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
/// Set
///

impl Imp<Set> for VisitableTrait {
    fn strategy(node: &Set) -> Option<TraitStrategy> {
        let inner = quote! {
            for (i, v) in self.0.iter().enumerate() {
                perform_visit(visitor, v, i);
            }
        };

        let q = quote_drive(&inner); // only immutable

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
        let mut inner_mut = quote!();

        for (i, _) in node.values.iter().enumerate() {
            let key = LitStr::new(&i.to_string(), Span::call_site());
            let index = syn::Index::from(i);

            inner.extend(quote! {
                perform_visit(visitor, &self.#index, #key);
            });

            inner_mut.extend(quote! {
                perform_visit_mut(visitor, &mut self.#index, #key);
            });
        }

        let q = quote_drives(&inner, &inner_mut);

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
    let mut inner_mut = quote!();

    for f in fields {
        let field_ident = format_ident!("{}", f.ident);
        let field_ident_s = field_ident.to_string();

        inner.extend(quote! {
            perform_visit(visitor, &self.#field_ident, #field_ident_s);
        });

        inner_mut.extend(quote! {
            perform_visit_mut(visitor, &mut self.#field_ident, #field_ident_s);
        });
    }

    quote_drives(&inner, &inner_mut)
}

// enum_variant
pub fn enum_variant(variant: &EnumVariant) -> (TokenStream, TokenStream) {
    let name = &variant.name;
    if variant.value.is_some() {
        let name_str = name.to_string();
        (
            quote! { Self::#name(value) => perform_visit(visitor, value, #name_str), },
            quote! { Self::#name(value) => perform_visit_mut(visitor, value, #name_str), },
        )
    } else {
        (quote! { Self::#name => {} }, quote! { Self::#name => {} })
    }
}

///
/// HELPERS
///

fn quote_drives(inner: &TokenStream, inner_mut: &TokenStream) -> TokenStream {
    let q = quote_drive(inner);
    let qm = quote_drive_mut(inner_mut);

    quote! {
        #q
        #qm
    }
}

// quote_drive
// (immutable)
fn quote_drive(inner: &TokenStream) -> TokenStream {
    quote! {
        fn drive(&self, visitor: &mut dyn ::mimic::core::visit::Visitor) {
            use ::mimic::core::visit::perform_visit;
            #inner
        }
    }
}

// quote_drive_mut
// (mutable)
fn quote_drive_mut(inner: &TokenStream) -> TokenStream {
    quote! {
        fn drive_mut(&mut self, visitor: &mut dyn ::mimic::core::visit::VisitorMut) {
            use ::mimic::core::visit::perform_visit_mut;
            #inner
        }
    }
}
