use crate::prelude::*;

///
/// EditViewTrait
///

pub struct EditViewTrait {}

impl Imp<Entity> for EditViewTrait {
    fn strategy(node: &Entity) -> Option<TraitStrategy> {
        Some(edit_impl(node, |n| {
            n.iter_editable_fields().map(|f| f.ident.clone()).collect()
        }))
    }
}

impl Imp<Record> for EditViewTrait {
    fn strategy(node: &Record) -> Option<TraitStrategy> {
        Some(edit_impl(node, |n| {
            n.fields.iter().map(|f| f.ident.clone()).collect()
        }))
    }
}

/// Shared generator
fn edit_impl<N, F>(node: &N, iter_fields: F) -> TraitStrategy
where
    N: HasViews,
    F: Fn(&N) -> Vec<syn::Ident>,
{
    let edit_ident = node.edit_ident();
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
        type EditType = #edit_ident;

        fn merge(&mut self, view: Self::EditType) {
            #(#merge_pairs)*
        }
    };

    let tokens = Implementor::new(node.def(), Trait::EditView)
        .set_tokens(q)
        .to_token_stream();

    TraitStrategy::from_impl(tokens)
}
