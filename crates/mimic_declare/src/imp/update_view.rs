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
        let def = node.def();
        let update_ident = node.update_ident();

        // generate merge pairs for every updatable field
        let merge_pairs: Vec<_> = node
            .iter_without_pk()
            .map(|field| {
                let ident = &field.ident;
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
                use ::mimic::core::traits::TypeView;
                #(#merge_pairs)*
            }
        };

        let tokens = Implementor::new(def, Trait::UpdateView)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}
