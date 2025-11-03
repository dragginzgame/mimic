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
    N: HasViews + HasDef,
    F: Fn(&N) -> Vec<syn::Ident>,
{
    let update_ident = node.update_ident();
    let field_idents = iter_fields(node);

    let merge_pairs: Vec<_> = field_idents
        .iter()
        .map(|ident| {
            quote! {
                if let Some(value) = view.#ident {
                    self.#ident = ::mimic::core::traits::View::from_view(value);
                }
            }
        })
        .collect();

    let q = quote! {
        type UpdateType = #update_ident;

        fn merge(&mut self, view: Self::UpdateType) {
            #(#merge_pairs)*
        }
    };

    let update_impl = Implementor::new(node.def(), Trait::UpdateView)
        .set_tokens(q)
        .to_token_stream();

    let tokens = quote! {
        #update_impl
    };

    TraitStrategy::from_impl(tokens)
}
