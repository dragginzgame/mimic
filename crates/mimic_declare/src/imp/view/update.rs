use crate::prelude::*;

///
/// UpdateViewTrait
///

pub struct UpdateViewTrait {}

impl Imp<Entity> for UpdateViewTrait {
    fn strategy(node: &Entity) -> Option<TraitStrategy> {
        Some(update_impl(node, |n| {
            n.iter_editable_fields().map(|f| f.ident.clone()).collect()
        }))
    }
}

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
    N: HasType,
    F: Fn(&N) -> Vec<syn::Ident>,
{
    let update_ident = node.update_ident();
    let field_idents = iter_fields(node);

    let merge_pairs: Vec<_> = field_idents
        .iter()
        .map(|ident| {
            quote! {
                if let Some(value) = view.#ident {
                    ::mimic::core::traits::UpdateView::merge(&mut self.#ident, value);
                }
            }
        })
        .collect();

    let q = quote! {
        type UpdateViewType = #update_ident;

        fn merge(&mut self, view: Self::UpdateViewType) {
            #(#merge_pairs)*
        }
    };

    let update_impl = Implementor::new(node.def(), TraitKind::UpdateView)
        .set_tokens(q)
        .to_token_stream();

    let tokens = quote! {
        #update_impl
    };

    TraitStrategy::from_impl(tokens)
}

///
/// Enum
///

impl Imp<Enum> for UpdateViewTrait {
    fn strategy(node: &Enum) -> Option<TraitStrategy> {
        let update_ident = node.update_ident();

        let q = quote! {
            type UpdateViewType = #update_ident;

            fn merge(&mut self, v: Self::UpdateViewType) {
                *self = v.into();
            }
        };

        Some(TraitStrategy::from_impl(
            Implementor::new(node.def(), TraitKind::UpdateView)
                .set_tokens(q)
                .to_token_stream(),
        ))
    }
}

///
/// List
///

impl Imp<List> for UpdateViewTrait {
    fn strategy(node: &List) -> Option<TraitStrategy> {
        let update_ident = node.update_ident();

        let q = quote! {
            type UpdateViewType = #update_ident;

            fn merge(&mut self, view: Self::UpdateViewType) {
                // Vec<T>: UpdateView is implemented generically.
                ::mimic::core::traits::UpdateView::merge(self, view);
            }
        };

        let update_impl = Implementor::new(node.def(), TraitKind::UpdateView)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(update_impl))
    }
}

///
/// Map
/// TODO - this was too complicated for that time of noight
///

impl Imp<Map> for UpdateViewTrait {
    fn strategy(node: &Map) -> Option<TraitStrategy> {
        let update_ident = node.update_ident();
        //      let value_ty = node.value.item.target().type_expr();

        let q = quote! {
            type UpdateViewType = #update_ident;

            fn merge(&mut self, view: Self::UpdateViewType) {
            }
        };

        Some(TraitStrategy::from_impl(
            Implementor::new(node.def(), TraitKind::UpdateView)
                .set_tokens(q)
                .to_token_stream(),
        ))
    }
}

///
/// Newtype
///

impl Imp<Newtype> for UpdateViewTrait {
    fn strategy(node: &Newtype) -> Option<TraitStrategy> {
        let update_ident = node.update_ident();

        let q = quote! {
            type UpdateViewType = #update_ident;

            fn merge(&mut self, view: Self::UpdateViewType) {
                // Delegate to inner
                ::mimic::core::traits::UpdateView::merge(&mut self.0, view);
            }
        };

        Some(TraitStrategy::from_impl(
            Implementor::new(node.def(), TraitKind::UpdateView)
                .set_tokens(q)
                .to_token_stream(),
        ))
    }
}

///
/// Set
///

impl Imp<Set> for UpdateViewTrait {
    fn strategy(node: &Set) -> Option<TraitStrategy> {
        let update_ident = node.update_ident();

        let q = quote! {
            type UpdateViewType = #update_ident;

            fn merge(&mut self, view: Self::UpdateViewType) {
                ::mimic::core::traits::UpdateView::merge(self, view);
            }
        };

        Some(TraitStrategy::from_impl(
            Implementor::new(node.def(), TraitKind::UpdateView)
                .set_tokens(q)
                .to_token_stream(),
        ))
    }
}

///
/// Tuple
///

impl Imp<Tuple> for UpdateViewTrait {
    fn strategy(node: &Tuple) -> Option<TraitStrategy> {
        let update_ident = node.update_ident();
        let values = &node.values;

        let merge_parts = values.iter().enumerate().map(|(i, _)| {
            let idx = syn::Index::from(i);
            quote! {
                if let Some(update) = view.#idx {
                    ::mimic::core::traits::UpdateView::merge(&mut self.#idx, update);
                }
            }
        });

        let q = quote! {
            type UpdateViewType = #update_ident;

            fn merge(&mut self, view: Self::UpdateViewType) {
                #(#merge_parts)*
            }
        };

        Some(TraitStrategy::from_impl(
            Implementor::new(node.def(), TraitKind::UpdateView)
                .set_tokens(q)
                .to_token_stream(),
        ))
    }
}
