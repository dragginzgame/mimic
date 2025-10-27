use crate::prelude::*;

///
/// UpdateViewTrait
///
pub struct UpdateViewTrait {}

///
/// Entity
///

impl Imp<Entity> for UpdateViewTrait {
    fn strategy(node: &Entity) -> Option<TraitStrategy> {
        Some(update_impl(node, |n| {
            n.iter_editable_fields().map(|f| f.ident.clone()).collect()
        }))
    }
}

///
/// Record
///

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
    let update_ident = node.update_ident();
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
        type View = #update_ident;

        fn merge(&mut self, view: Self::View) {
            #(#merge_pairs)*
        }
    };

    let tokens = Implementor::new(def, Trait::UpdateView)
        .set_tokens(q)
        .to_token_stream();

    TraitStrategy::from_impl(tokens)
}
