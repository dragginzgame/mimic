mod type_view;

pub use type_view::*;

use crate::prelude::*;

///
/// CreateViewTrait
///

pub struct CreateViewTrait {}

/// Entity
impl Imp<Entity> for CreateViewTrait {
    fn strategy(node: &Entity) -> Option<TraitStrategy> {
        let view_ident = &node.create_ident();

        // tokens
        let q = quote! {
            type View = #view_ident;
        };

        let tokens = Implementor::new(node.def(), Trait::CreateView)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// UpdateViewTrait
///

pub struct UpdateViewTrait {}

/// Entity
impl Imp<Entity> for UpdateViewTrait {
    fn strategy(node: &Entity) -> Option<TraitStrategy> {
        Some(update_impl(node, |n| {
            n.iter_editable_fields().map(|f| f.ident.clone()).collect()
        }))
    }
}

/// Record
impl Imp<Record> for UpdateViewTrait {
    fn strategy(node: &Record) -> Option<TraitStrategy> {
        Some(update_impl(node, |n| {
            n.fields.iter().map(|f| f.ident.clone()).collect()
        }))
    }
}

/// Shared generator
fn update_impl<N, F>(node: &N, iter_fields: F) -> TraitStrategy
where
    N: HasDef + HasViewTypes,
    F: Fn(&N) -> Vec<syn::Ident>,
{
    let def = node.def();
    let view_ident = node.update_ident();
    let field_idents = iter_fields(node);

    let merge_pairs: Vec<_> = field_idents
        .iter()
        .map(|ident| {
            quote! {
                if let Some(value) = view.#ident {
                    self.#ident = ::mimic::core::traits::TypeView::from_view(value);
                }
            }
        })
        .collect();

    let q = quote! {
        type View = #view_ident;

        fn merge(&mut self, view: Self::View) {
            #(#merge_pairs)*
        }
    };

    let tokens = Implementor::new(def, Trait::UpdateView)
        .set_tokens(q)
        .to_token_stream();

    TraitStrategy::from_impl(tokens)
}

///
/// FilterViewTrait
///

pub struct FilterViewTrait {}

/// Entity
impl Imp<Entity> for FilterViewTrait {
    fn strategy(node: &Entity) -> Option<TraitStrategy> {
        let view_ident = &node.filter_ident();

        // tokens
        let q = quote! {
            type View = #view_ident;
        };

        let tokens = Implementor::new(node.def(), Trait::FilterView)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// SortViewTrait
///

pub struct SortViewTrait {}

/// Entity
impl Imp<Entity> for SortViewTrait {
    fn strategy(node: &Entity) -> Option<TraitStrategy> {
        let view_ident = &node.sort_ident();

        // tokens
        let q = quote! {
            type View = #view_ident;
        };

        let tokens = Implementor::new(node.def(), Trait::SortView)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}
