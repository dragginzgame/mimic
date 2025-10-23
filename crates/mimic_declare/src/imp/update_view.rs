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
        let update_ident = &node.update_ident();

        // tokens
        let q = quote! {
            type View = #update_ident;
        };

        let tokens = Implementor::new(node.def(), Trait::UpdateView)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}
